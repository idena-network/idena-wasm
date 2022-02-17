use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};

use wasmer::{HostEnvInitError, Instance, Memory, Val, WasmerEnv};
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints, set_remaining_points};

use crate::backend::Backend;
use crate::errors;
use crate::errors::VmError;
use crate::memory::VmResult;

#[derive(Debug)]
pub enum Never {}

pub struct Env<B: Backend> {
    pub backend: B,
    data: Arc<RwLock<ContextData>>,
}


impl<B: Backend> Env<B> {
    pub fn new(api: B) -> Self {
        Env {
            backend: api,
            data: Arc::new(RwLock::new(ContextData::new())),
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

    fn call_function(&self, name: &str, args: &[Val]) -> VmResult<Box<[Val]>> {
        // Clone function before calling it to avoid dead locks
        let func = self.with_wasmer_instance(|instance| {
            let func = instance.exports.get_function(name)?;
            Ok(func.clone())
        })?;
        func.call(args).map_err(|runtime_err| -> VmError {
            self.with_wasmer_instance::<_, Never>(|instance| {
                let err: VmError = match get_remaining_points(instance) {
                    MeteringPoints::Remaining(_) => VmError::custom(runtime_err.to_string()),
                    MeteringPoints::Exhausted => VmError::custom("Ran out of gas during contract execution"),
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
}

impl<B: Backend> Clone for Env<B> {
    fn clone(&self) -> Self {
        Env {
            backend: self.backend,
            data: self.data.clone(),
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
}

impl ContextData {
    pub fn new() -> Self {
        ContextData {
            wasmer_instance: None,
        }
    }
}
