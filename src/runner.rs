use std::borrow::Borrow;
use std::ptr::NonNull;
use std::sync::Arc;

use indexmap::map::Iter;
use protobuf::{Message, RepeatedField, SingularPtrField};
use wasmer::{BaseTunables, ChainableNamedResolver, CompilerConfig, Exportable, ExportIndex, Exports, Function, ImportObject, imports, Instance, Module, Pages, Resolver, Singlepass, Store, Target, Val, Value};
use wasmer::wasmparser::Operator;
use wasmer_engine_universal::Universal;
use wasmer_middlewares::Metering;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints};
use wasmer_types::ModuleInfo;

use crate::{unwrap_or_action_res, unwrap_or_return};
use crate::args::convert_args;
use crate::backend::{Backend, BackendError, BackendResult};
use crate::costs::*;
use crate::environment::Env;
use crate::errors::VmError;
use crate::gatekeeper::*;
use crate::imports::*;
use crate::limiting_tunables::LimitingTunables;
use crate::memory::{read_region, VmResult};
use crate::proto::models::{InvocationContext as protoContext, ProtoArgs_Argument};
use crate::types::{Action, ActionResult, Address, DeployContractAction, FunctionCallAction, Gas, InvocationContext, Promise, PromiseResult, ReadContractDataAction, ReadShardedDataAction};
use crate::types::PromiseResult::Failed;

pub struct VmRunner<B: Backend + 'static> {
    pub contact_addr: Address,
    pub api: B,
    pub gas_limit: Gas,
    ctx: Option<InvocationContext>,
    pub is_debug: bool,
}

impl<B: Backend + 'static> VmRunner<B> {
    pub fn new(api: B, contract_addr: Address, gas_limit: Gas, ctx: Option<InvocationContext>, is_debug: bool) -> Self {
        VmRunner {
            contact_addr: contract_addr,
            api,
            gas_limit,
            ctx,
            is_debug,
        }
    }

    fn prepare_arguments(&self, env: &Env<B>, info: &ModuleInfo, method: &String, args: protobuf::RepeatedField<ProtoArgs_Argument>) -> VmResult<Vec<Val>> {
        let mut params_cnt = 0;

        if self.is_debug {
            let exp_it: Iter<'_, String, ExportIndex> = info.exports.iter();

            for k in exp_it {
                println!("export [{}]={:?}", k.0, k.1)
            }
        }

        match info.exports.get(method) {
            Some(ExportIndex::Function(index)) => {
                let func = info.functions.get(index.clone()).unwrap();
                let sign = info.signatures.get(func.clone()).unwrap();
                params_cnt = sign.params().len();
            }
            None => return Err(VmError::custom("method is not found")),
            _ => return Err(VmError::custom("method is not found"))
        };

        if params_cnt < args.len() {
            return Err(VmError::custom("too many arguments"));
        }

        let mut wasm_args = Vec::new();
        for v in args.iter() {
            if v.get_is_nil() {
                wasm_args.push(Value::I32(0));
                continue;
            }
            match write_to_contract(&env.clone(), v.get_value()) {
                Ok(p) => wasm_args.push(Value::I32(i32::try_from(p).unwrap())),
                Err(err) => return Err(err)
            }
        }

        while wasm_args.len() < params_cnt {
            wasm_args.push(Value::I32(0));
        }
        Ok(wasm_args)
    }

    fn build_env(&self, code: Vec<u8>, promise_result: Option<PromiseResult>) -> VmResult<(Env<B>, Module, Box<Instance>)> {
        let metering = Arc::new(Metering::new(self.gas_limit, cost_function));
        let mut compiler_config = Singlepass::default();
        compiler_config.push_middleware(metering);
        compiler_config.push_middleware(Arc::new(Gatekeeper::default()));
        let base = BaseTunables::for_target(&Target::default());
        let store = Store::new_with_tunables(&Universal::new(compiler_config).engine(), LimitingTunables::new(base, Pages(100)));
        let env = Env::new(self.api, promise_result);
        let mut import_object = imports! {
        "env" => {
            "abort" => Function::new_native_with_env(&store, env.clone(), abort),
            "panic" => Function::new_native_with_env(&store, env.clone(), panic),
            "set_storage" => Function::new_native_with_env(&store, env.clone(), set_storage),
            "get_storage" => Function::new_native_with_env(&store, env.clone(), get_storage),
            "remove_storage" => Function::new_native_with_env(&store, env.clone(), remove_storage),
            "block_timestamp" => Function::new_native_with_env(&store, env.clone(), block_timestamp),
            "block_number" => Function::new_native_with_env(&store, env.clone(), block_number),
            "block_seed" => Function::new_native_with_env(&store, env.clone(), block_seed),
            "min_fee_per_gas" => Function::new_native_with_env(&store, env.clone(), min_fee_per_gas),
            "network_size" => Function::new_native_with_env(&store, env.clone(), network_size),
            "caller" => Function::new_native_with_env(&store, env.clone(), caller),
            "original_caller" => Function::new_native_with_env(&store, env.clone(), original_caller),
            "create_call_function_promise" => Function::new_native_with_env(&store, env.clone(), create_call_function_promise),
            "create_deploy_contract_promise" => Function::new_native_with_env(&store, env.clone(), create_deploy_contract_promise),
            "create_read_contract_data_promise" => Function::new_native_with_env(&store, env.clone(), create_read_contract_data_promise),
            "create_get_identity_promise" => Function::new_native_with_env(&store, env.clone(), create_get_identity_promise),
            "create_transfer_promise" => Function::new_native_with_env(&store, env.clone(), create_transfer_promise),
            "promise_result" => Function::new_native_with_env(&store, env.clone(), crate::imports::promise_result),
            "promise_then" => Function::new_native_with_env(&store, env.clone(), promise_then),
            "own_addr" => Function::new_native_with_env(&store, env.clone(), own_addr),
            "own_code" => Function::new_native_with_env(&store, env.clone(), own_code),
            "contract_addr" => Function::new_native_with_env(&store, env.clone(), contract_addr),
            "contract_addr_by_hash" => Function::new_native_with_env(&store, env.clone(), contract_addr_by_hash),
            "code_hash" => Function::new_native_with_env(&store, env.clone(), code_hash),
            "emit_event" => Function::new_native_with_env(&store, env.clone(), event),
            "epoch" => Function::new_native_with_env(&store, env.clone(), epoch),
            "pay_amount" => Function::new_native_with_env(&store, env.clone(), pay_amount),
            "bytes_to_hex" => Function::new_native_with_env(&store, env.clone(), bytes_to_hex),
            }
        };
        let mut import_obj_debug = imports! {};
        if self.is_debug {
            import_obj_debug = imports! {
            "env" => {
                   "debug" =>  Function::new_native_with_env(&store, env.clone(), debug)
            }
        };
        }
        let resolver = import_obj_debug.chain_back(import_object);
        let module = match Module::new(&store, code) {
            Ok(v) => v,
            Err(err) => {
                return Err(VmError::custom(format!("compilation error: {:?}", err)));
            }
        };

        let instance = Instance::new(&module, &resolver)?;

        let wasmer_instance = Box::from(instance);

        let instance_ptr = NonNull::from(wasmer_instance.as_ref());
        env.set_wasmer_instance(Some(instance_ptr));
        Ok((env, module, wasmer_instance))
    }


    pub fn apply_function_call(&self, contract: Address, action: &FunctionCallAction, promise_result: Option<PromiseResult>, gas_used: &mut u64, is_callback: bool) -> VmResult<ActionResult> {
        let ctx = InvocationContext {
            is_callback: is_callback,
            promise_result: promise_result,
        };
        let (res, gas) = self.api.call(contract, action.method_name.as_bytes(), &action.args, &action.deposit, action.gas_limit, &Into::<protoContext>::into(ctx).write_to_bytes().unwrap_or_default());
        *gas_used = gas;
        Ok(res?)
    }

    pub fn execute_promises(&self, env: Env<B>) -> Vec<ActionResult> {
        let promises = env.get_promises();
        if self.is_debug {
            println!("execute promises cnt={}", promises.len());
        }
        if promises.is_empty() {
            return Vec::new();
        }
        let iter = promises.iter();
        let mut result: Vec<ActionResult> = Vec::with_capacity(iter.len());

        for p in iter {
            match &p.action {
                Action::FunctionCall(call) => {
                    let mut gas_used = 0;
                    let action_result = self.apply_function_call(p.receiver_id.to_vec(), call, None, &mut gas_used, false);

                    let promise_result = match action_result {
                        Ok(action_res) => {
                            result.push(action_res.clone());
                            if !action_res.success {
                                Some(PromiseResult::Failed)
                            } else if action_res.output_data.is_empty() {
                                Some(PromiseResult::Empty)
                            } else {
                                Some(PromiseResult::Value(action_res.output_data))
                            }
                        }
                        Err(err) => {
                            if !call.deposit.is_empty() {
                                // refund deposit
                                self.api.add_balance(p.predecessor_id.to_vec(), call.deposit.to_vec());
                                self.api.commit();
                            }
                            result.push(Self::action_result_from_err(err, p.receiver_id.clone(), p.action.clone(), gas_used, call.gas_limit));
                            Some(PromiseResult::Failed)
                        }
                    };

                    self.run_callback(&mut result, p, promise_result)
                }
                Action::DeployContract(deploy) => {
                    let mut gas_used = 0;

                    let (action_result, gas) = self.api.deploy(&deploy.code, &deploy.args, &deploy.nonce, &deploy.deposit, deploy.gas_limit);
                    gas_used = gas;

                    let promise_result = match action_result {
                        Ok(action_res) => {
                            result.push(action_res.clone());
                            if !action_res.success {
                                Some(PromiseResult::Failed)
                            } else if action_res.output_data.is_empty() {
                                Some(PromiseResult::Empty)
                            } else {
                                Some(PromiseResult::Value(action_res.output_data))
                            }
                        }
                        Err(err) => {
                            if !deploy.deposit.is_empty() {
                                // refund deposit
                                self.api.add_balance(p.predecessor_id.to_vec(), deploy.deposit.to_vec());
                                self.api.commit();
                            }
                            result.push(Self::action_result_from_err(err.into(), p.receiver_id.clone(), p.action.clone(), gas_used, deploy.gas_limit));
                            Some(PromiseResult::Failed)
                        }
                    };

                    self.run_callback(&mut result, p, promise_result)
                }
                Action::Transfer(t) => {
                    //todo : handle result
                    self.api.add_balance(p.receiver_id.clone(), t.amount.to_vec());
                }
                Action::ReadShardedData(read_shared_data_action) => {
                    match read_shared_data_action {
                        ReadShardedDataAction::ReadContractData(req) => {
                            let action_result = self.api.read_contract_data(p.receiver_id.clone(), req.key.clone());
                            let promise_result = self.execute_read_sharded_data(action_result, p.receiver_id.clone(), &mut result, p.action.clone(), req.gas_limit);
                            self.run_callback(&mut result, p, promise_result)
                        }
                        ReadShardedDataAction::GetIdentity(req)
                        => {
                            let action_result = self.api.identity(req.addr.clone());
                            let promise_result = self.execute_read_sharded_data(action_result, p.receiver_id.clone(), &mut result, p.action.clone(), req.gas_limit);
                            self.run_callback(&mut result, p, promise_result)
                        }
                    }
                }
                _ => {}
            };
        };
        result
    }

    fn execute_read_sharded_data(&self, action_res: BackendResult<Option<Vec<u8>>>, address: Address, result: &mut Vec<ActionResult>, action: Action, gas_limit: u64) -> Option<PromiseResult> {
        let (action_result, gas) = action_res;
        let mut gas_used = gas;
        let mut promise_result: Option<PromiseResult>;
        if gas_used > gas_limit {
            let res = Self::action_result_from_err(VmError::OutOfGas, address, action, gas_limit, gas_limit);
            result.push(res);
            promise_result = Some(PromiseResult::Failed)
        } else {
            match action_result {
                Ok(data) => {
                    let res = match data {
                        None => {
                            promise_result = Some(PromiseResult::Empty);
                            Self::action_result_from_success(action, address, vec![], gas_used, gas_limit)
                        }
                        Some(v) => {
                            promise_result = Some(PromiseResult::Value(v.clone()));
                            Self::action_result_from_success(action, address, v, gas_used, gas_limit)
                        }
                    };
                    result.push(res);
                }
                Err(err) => {
                    let res = Self::action_result_from_err(err.into(), address, action, gas_used, gas_limit);
                    result.push(res);
                    promise_result = Some(PromiseResult::Failed);
                }
            }
        }
        promise_result
    }


    fn run_callback(&self, result: &mut Vec<ActionResult>, p: &Promise, promise_result: Option<PromiseResult>) {
        if p.action_callback.is_some() {
            let action = p.action_callback.as_ref().unwrap().clone();
            match action {
                Action::FunctionCall(ref call) => {
                    let mut gas_used = 0;
                    let action_result = self.apply_function_call(self.contact_addr.clone(), &call, promise_result, &mut gas_used, true);
                    if action_result.is_err() && !call.deposit.is_empty() {
                        // refund deposit
                        self.api.add_balance(p.predecessor_id.to_vec(), call.deposit.to_vec());
                        self.api.commit();
                    }
                    result.push(action_result.unwrap_or_else(|err| Self::action_result_from_err(err, self.contact_addr.clone(), action.clone(), gas_used, call.gas_limit)));
                }
                _ => unreachable!(),
            };
        }
    }
    pub fn deploy(&self, code: Vec<u8>, arg_bytes: &[u8], gas_used: &mut u64) -> ActionResult {
        *gas_used = BASE_DEPLOY_COST;
        let input_action = Action::DeployContract(DeployContractAction {
            gas_limit: self.gas_limit,
            deposit: vec![],
            args: arg_bytes.to_vec(),
            nonce: vec![],
            code: vec![], // drop code
        });
        let addr = self.contact_addr.clone();
        let (env, module, instance) = unwrap_or_action_res!(self.build_env(code, None), input_action, *gas_used, self.gas_limit, addr);
        unwrap_or_action_res!(process_gas_info(&env, BASE_DEPLOY_COST), input_action, *gas_used, self.gas_limit, self.contact_addr.clone());
        unwrap_or_action_res!(self.deploy_with_env(env, module, input_action.clone(),  arg_bytes, gas_used), input_action, *gas_used, self.gas_limit, addr)
    }
    pub fn deploy_with_env(&self, env: Env<B>, module: Module, input_action: Action, arg_bytes: &[u8], gas_used: &mut u64) -> VmResult<ActionResult> {
        let args = convert_args(arg_bytes)?;

        let required_export = ["allocate", "deploy", "memory"];
        let module_info = module.info();
        for export in required_export {
            match module_info.exports.get(export) {
                Some(_) => continue,
                None => return Err(VmError::custom(format!("not found required export: {}", export)))
            }
        }

        let wasm_args = self.prepare_arguments(&env.clone(), module.info(), &"deploy".to_string(), args)?;

        let res = env.call_function("deploy", &wasm_args);

        *gas_used = self.gas_limit.saturating_sub(env.get_gas_left());

        if res.is_err() {
            return Ok(Self::action_result_from_err(res.err().unwrap(), self.contact_addr.clone(), input_action, *gas_used, self.gas_limit));
        }

        let mut res = Self::action_result_from_success(input_action, self.contact_addr.clone(), vec![], *gas_used, self.gas_limit);
        res.append_sub_action_results(self.execute_promises(env));

        let gas_refund = res.sub_action_results.iter().fold(0, |a, x| a + x.remaining_gas);
        if gas_refund > 0 {
            *gas_used -= gas_refund;
        }
        res.gas_used = *gas_used;
        Ok(res)
    }

    pub fn execute(self, code: Vec<u8>, method: &String, arg_bytes: &[u8], gas_used: &mut u64) -> ActionResult {
        let input_action = Action::FunctionCall(FunctionCallAction {
            gas_limit: self.gas_limit,
            deposit: vec![],
            args: arg_bytes.to_vec(),
            method_name: method.to_string(),
        });
        let invocation_ctx = self.ctx.clone().unwrap_or_default();
        let (env, module, instance) = unwrap_or_action_res!(self.build_env(code, invocation_ctx.promise_result),
            input_action,  *gas_used, self.gas_limit,  self.contact_addr.clone());
        unwrap_or_action_res!(process_gas_info(&env, BASE_CALL_COST), input_action, *gas_used, self.gas_limit, self.contact_addr.clone());
        unwrap_or_action_res!(self.execute_with_env(env, module, input_action.clone(), method, arg_bytes, gas_used, invocation_ctx.is_callback), input_action, *gas_used, self.gas_limit,  self.contact_addr.clone())
    }

    pub fn execute_with_env(&self, env: Env<B>, module: Module, input_action: Action, method: &String, arg_bytes: &[u8], gas_used: &mut u64, is_callback: bool) -> VmResult<ActionResult> {
        if method == "deploy" {
            return Err(VmError::custom("direct call to deploy is forbidden'"));
        }
        if !is_callback && method.starts_with("_") {
            return Err(VmError::custom("direct call to promise callback is forbidden'"));
        }

        let args = convert_args(arg_bytes)?;

        let wasm_args = self.prepare_arguments(&env.clone(), module.info(), &method, args)?;
        let mut output_data = vec![];
        let res = match env.call_function(method.as_str(), &wasm_args) {
            Ok(val) => {
                let mut ptr = 0;
                if !val.is_empty() {
                    ptr = match val[0] {
                        Value::I32(v) => v,
                        Value::I64(v) => v as i32,
                        _ => 0
                    };
                }
                if ptr > 0 {
                    output_data = read_region(&env.memory(), ptr as u32, MAX_RETURN_VALUE_SIZE).unwrap_or(vec![]);
                }
                *gas_used = self.gas_limit.saturating_sub(env.get_gas_left());
                Ok(())
            }
            Err(err) => {
                *gas_used = self.gas_limit.saturating_sub(env.get_gas_left());
                Err(err)
            }
        };

        if res.is_err() {
            return Ok(Self::action_result_from_err(res.err().unwrap(), self.contact_addr.clone(), input_action, *gas_used, self.gas_limit));
        }

        let mut res = Self::action_result_from_success(input_action, self.contact_addr.clone(), output_data, *gas_used, self.gas_limit);
        res.append_sub_action_results(self.execute_promises(env));

        let gas_refund = res.sub_action_results.iter().fold(0, |a, x| a + x.remaining_gas);
        if gas_refund > 0 {
            *gas_used -= gas_refund;
        }
        res.gas_used = *gas_used;
        if self.is_debug {
            println!("action result={:?}", res);
        }
        Ok(res)
    }


    fn action_result_from_err(err: VmError, contract: Address, input_action: Action, gas_used: u64, gas_limit: u64) -> ActionResult {
        ActionResult {
            contract: contract,
            error: err.to_string(),
            success: false,
            gas_used: gas_used,
            remaining_gas: gas_limit.saturating_sub(gas_used),
            input_action: input_action,
            sub_action_results: vec![],
            output_data: vec![],
        }
    }

    fn action_result_from_success(input_action: Action, contract: Address, output_data: Vec<u8>, gas_used: u64, gas_limit: u64) -> ActionResult {
        ActionResult {
            contract: contract,
            error: String::new(),
            success: true,
            gas_used: gas_used,
            remaining_gas: gas_limit.saturating_sub(gas_used),
            input_action: input_action,
            sub_action_results: vec![],
            output_data: output_data,
        }
    }
}

