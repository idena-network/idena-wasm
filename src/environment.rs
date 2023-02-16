use std::borrow::{Borrow, BorrowMut};
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};

use wasmer::{HostEnvInitError, Instance, Memory, Val, WasmerEnv};
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints, set_remaining_points};

use crate::{unwrap_or_return};
use crate::backend::{Backend, BackendError, BackendResult};
use crate::costs::BASE_PROMISE_COST;
use crate::errors::VmError;
use crate::memory::VmResult;
use crate::types::{Address, DeployContractAction, IDNA, ReadShardedDataAction};
use crate::types::{Action, FunctionCallAction, Promise, PromiseResult, TransferAction};

#[derive(Debug)]
pub enum Never {}

pub struct Env<B: Backend> {
    pub backend: B,
    data: Arc<RwLock<ContextData>>,
    pub promise_result: Option<PromiseResult>,
    gas_limit : u64,
}

impl<B: Backend> Env<B> {
    pub fn new(api: B, promise_res: Option<PromiseResult>, gas_limit : u64) -> Self {
        Env {
            backend: api,
            data: Arc::new(RwLock::new(ContextData::new())),
            promise_result: promise_res,
            gas_limit : gas_limit,
        }
    }

    pub fn set_wasmer_instance(&self, wasmer_instance: Option<NonNull<Instance>>) {
        self.with_context_data_mut(|context_data| {
            context_data.wasmer_instance = wasmer_instance;
        });
    }

    fn with_context_data_mut<C, R>(&self, callback: C) -> R
        where
            C: FnOnce(&mut ContextData) -> R,
    {
        let mut guard = self.data.as_ref().write().unwrap();
        let context_data = guard.borrow_mut();
        callback(context_data)
    }

    fn with_context_data<C, R>(&self, callback: C) -> R
        where
            C: FnOnce(&ContextData) -> R,
    {
        let guard = self.data.as_ref().read().unwrap();
        let context_data = guard.borrow();
        callback(context_data)
    }

    pub fn with_wasmer_instance<C, R>(&self, callback: C) -> Result<R, VmError>
        where
            C: FnOnce(&Instance) -> Result<R, VmError>,
    {
        self.with_context_data(|context_data| match context_data.wasmer_instance {
            Some(instance_ptr) => {
                let instance_ref = unsafe { instance_ptr.as_ref() };
                callback(instance_ref)
            }
            None => Err(VmError::custom("uninitialized wasmer instance")),
        })
    }

    pub fn get_gas_left(&self) -> u64 {
        self.with_wasmer_instance(|instance| {
            Ok(match get_remaining_points(instance) {
                MeteringPoints::Remaining(count) => count,
                MeteringPoints::Exhausted => 0,
            })
        })
            .expect("Wasmer instance is not set. This is a bug in the lifecycle.")
    }

    pub fn gas_limit(&self) -> u64 {
        return self.gas_limit;
    }

    pub fn set_gas_left(&self, new_value: u64) {
        self.with_wasmer_instance(|instance| {
            set_remaining_points(instance, new_value);
            Ok(())
        })
            .expect("Wasmer instance is not set. This is a bug in the lifecycle.")
    }

    pub fn memory(&self) -> Memory {
        self.with_wasmer_instance(|instance| {
            let first: Option<Memory> = instance
                .exports
                .iter()
                .memories()
                .next()
                .map(|pair| pair.1.clone());
            // Every contract in CosmWasm must have exactly one exported memory.
            // This is ensured by `check_wasm`/`check_wasm_memories`, which is called for every
            // contract added to the Cache as well as in integration tests.
            // It is possible to bypass this check when using `Instance::from_code` but then you
            // learn the hard way when this panics, or when trying to upload the contract to chain.
            let memory = first.expect("A contract must have exactly one exported memory.");
            Ok(memory)
        })
            .expect("Wasmer instance is not set. This is a bug in the lifecycle.")
    }

    pub fn call_function(&self, name: &str, args: &[Val]) -> VmResult<Box<[Val]>> {
        // Clone function before calling it to avoid dead locks
        let func = self.with_wasmer_instance(|instance| {
            let func = instance.exports.get_function(name)?;
            Ok(func.clone())
        })?;
        func.call(args).map_err(|runtime_err| -> VmError {
            self.with_wasmer_instance::<_, Never>(|instance| {
                let err: VmError = match get_remaining_points(instance) {
                    MeteringPoints::Remaining(_) => VmError::custom(runtime_err.to_string()),
                    MeteringPoints::Exhausted => VmError::out_of_gas(),
                };
                Err(err)
            })
                .unwrap_err() // with_wasmer_instance can only succeed if the callback succeeds
        })
    }

    pub fn call_function1(&self, name: &str, args: &[Val]) -> VmResult<Val> {
        let result = self.call_function(name, args)?;
        let expected = 1;
        let actual = result.len();
        if actual != expected {
            return Err(VmError::custom(format!("Unexpected number of result values when calling '{}'. Expected: {}, actual: {}.", name, expected, actual)));
        }
        Ok(result[0].clone())
    }

    pub fn create_transfer_promise(&self, to: Address, amount: IDNA) -> BackendResult<()> {
        let (own_addr_res, gas_used) = self.backend.own_addr();
        let own_addr = unwrap_or_return!(own_addr_res, gas_used);
        self.with_context_data_mut(|data| {
            data.pending_promises.push(Promise {
                predecessor_id: own_addr,
                receiver_id: to,
                action: Action::Transfer(TransferAction {
                    amount
                }),
                action_callback: None,
            })
        });
        (Ok(()), gas_used.saturating_add(BASE_PROMISE_COST))
    }


    pub fn create_function_call_promise(&self, to: Address, method: Vec<u8>, args: Vec<u8>, amount: IDNA, gas_limit: u64) -> BackendResult<u32> {
        let (own_addr_res, gas_used) = self.backend.own_addr();
        let own_addr = unwrap_or_return!(own_addr_res, gas_used);
        self.with_context_data_mut(|data| {
            data.pending_promises.push(Promise {
                predecessor_id: own_addr,
                receiver_id: to,
                action: Action::FunctionCall(FunctionCallAction {
                    gas_limit,
                    args,
                    method_name: String::from_utf8_lossy(&method).to_string(),
                    deposit: amount,
                }),
                action_callback: None,
            });
            (Ok(data.pending_promises.len() as u32 - 1), gas_used.saturating_add(BASE_PROMISE_COST))
        })
    }


    pub fn create_deploy_contract_promise(&self, code: Vec<u8>, args: Vec<u8>, nonce: Vec<u8>, amount: IDNA, gas_limit: u64) -> BackendResult<u32> {
        let (own_addr_res, mut gas_used) = self.backend.own_addr();
        let own_addr = unwrap_or_return!(own_addr_res, gas_used);

        let (to_res, gas) = self.backend.contract_addr(&code, &args, &nonce);
        gas_used += gas;
        let to = unwrap_or_return!(to_res, gas_used);

        self.with_context_data_mut(|data| {
            data.pending_promises.push(Promise {
                predecessor_id: own_addr,
                receiver_id: to,
                action: Action::DeployContract(DeployContractAction {
                    code,
                    nonce,
                    gas_limit,
                    args,
                    deposit: amount,
                }),
                action_callback: None,
            });
            (Ok(data.pending_promises.len() as u32 - 1), gas_used.saturating_add(BASE_PROMISE_COST))
        })
    }


    pub fn create_read_sharded_data_promise(&self, to: Address, action: ReadShardedDataAction) -> BackendResult<u32> {
        let (own_addr_res, gas_used) = self.backend.own_addr();
        let own_addr = unwrap_or_return!(own_addr_res, gas_used);
        self.with_context_data_mut(|data| {
            data.pending_promises.push(Promise {
                predecessor_id: own_addr,
                receiver_id: to,
                action: Action::ReadShardedData(action),
                action_callback: None,
            });
            (Ok(data.pending_promises.len() as u32 - 1), gas_used.saturating_add(BASE_PROMISE_COST))
        })
    }

    pub fn promise_then(&self, promise_idx: usize, method: Vec<u8>, args: Vec<u8>, amount: IDNA, gas_limit: u64) -> BackendResult<()> {
        self.with_context_data_mut(|data| {
            match data.pending_promises.get_mut(promise_idx) {
                Some(promise) => if promise.action_callback.is_some() {
                    return (Err(BackendError::new("promise is completed")), BASE_PROMISE_COST);
                } else {
                    promise.action_callback = Some(Action::FunctionCall(FunctionCallAction {
                        gas_limit,
                        args,
                        method_name: String::from_utf8_lossy(&method).to_string(),
                        deposit: amount,
                    }));
                    (Ok(()), BASE_PROMISE_COST)
                }
                None => (Err(BackendError::new("invalid promise_idx")), BASE_PROMISE_COST)
            }
        })
    }

    pub fn get_promises(&self) -> Vec<Promise> {
        let mut result = Vec::new();
        self.with_context_data_mut(|data| {
            result = data.pending_promises.to_vec()
        });
        result
    }
}

impl<B: Backend> Clone for Env<B> {
    fn clone(&self) -> Self {
        Env {
            backend: self.backend,
            data: self.data.clone(),
            promise_result: self.promise_result.clone(),
            gas_limit : self.gas_limit
        }
    }
}

unsafe impl<B: Backend> Send for Env<B> {}

unsafe impl<B: Backend> Sync for Env<B> {}

impl<B: Backend> WasmerEnv for Env<B> {
    fn init_with_instance(&mut self, _instance: &Instance) -> Result<(), HostEnvInitError> {
        Ok(())
    }
}

pub struct ContextData {
    wasmer_instance: Option<NonNull<Instance>>,
    pending_promises: Vec<Promise>,
}

impl ContextData {
    pub fn new() -> Self {
        ContextData {
            wasmer_instance: None,
            pending_promises: Vec::new(),
        }
    }
}
