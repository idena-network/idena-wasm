use std::borrow::Borrow;
use std::ptr::NonNull;
use std::sync::Arc;

use indexmap::map::Iter;
use protobuf::{RepeatedField, SingularPtrField};
use wasmer::{BaseTunables, CompilerConfig, Exportable, ExportIndex, Exports, Function, ImportObject, imports, Instance, Module, Pages, Singlepass, Store, Target, Val, Value};
use wasmer::wasmparser::Operator;
use wasmer_engine_universal::Universal;
use wasmer_middlewares::Metering;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints};
use wasmer_types::ModuleInfo;

use crate::args::convert_args;
use crate::backend::{Backend, BackendResult};
use crate::costs::*;
use crate::environment::Env;
use crate::errors::VmError;
use crate::gatekeeper::*;
use crate::imports::*;
use crate::limiting_tunables::LimitingTunables;
use crate::memory::{read_region, VmResult};
use crate::types::{Action, ActionResult, Address, FunctionCallAction, PromiseResult};
use crate::types::PromiseResult::Failed;

pub struct VmRunner {}

impl VmRunner {
    fn prepare_arguments<B: Backend + 'static>(env: &Env<B>, info: &ModuleInfo, method: &String, args: protobuf::RepeatedField<Vec<u8>>) -> VmResult<Vec<Val>> {
        let mut params_cnt = 0;

        let exp_it: Iter<'_, String, ExportIndex> = info.exports.iter();

        for k in exp_it {
            println!("export [{}]={:?}", k.0, k.1)
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
            if v.is_empty() {
                wasm_args.push(Value::I32(0));
                continue;
            }
            match write_to_contract(&env.clone(), v) {
                Ok(p) => wasm_args.push(Value::I32(i32::try_from(p).unwrap())),
                Err(err) => return Err(err)
            }
        }

        while wasm_args.len() < params_cnt {
            wasm_args.push(Value::I32(0));
        }
        Ok(wasm_args)
    }

    fn build_env<B: Backend + 'static>(api: B, code: Vec<u8>, promise_result: Option<PromiseResult>, gas_limit: u64) -> VmResult<(Env<B>, Module, Box<Instance>)> {
        let metering = Arc::new(Metering::new(gas_limit, cost_function));
        let mut compiler_config = Singlepass::default();
        compiler_config.push_middleware(metering);
        compiler_config.push_middleware(Arc::new(Gatekeeper::default()));
        let base = BaseTunables::for_target(&Target::default());
        let store = Store::new_with_tunables(&Universal::new(compiler_config).engine(), LimitingTunables::new(base, Pages(100)));
        let env = Env::new(api, promise_result);
        let import_object = imports! {
        "env" => {
            "debug" => Function::new_native_with_env(&store, env.clone(), debug),
            "abort" => Function::new_native_with_env(&store, env.clone(), abort),
            "panic" => Function::new_native_with_env(&store, env.clone(), panic),
            "set_storage" => Function::new_native_with_env(&store, env.clone(), set_storage),
            "get_storage" => Function::new_native_with_env(&store, env.clone(), get_storage),
            "remove_storage" => Function::new_native_with_env(&store, env.clone(), remove_storage),
            "block_timestamp" => Function::new_native_with_env(&store, env.clone(), block_timestamp),
            "block_number" => Function::new_native_with_env(&store, env.clone(), block_number),
            "min_fee_per_gas" => Function::new_native_with_env(&store, env.clone(), min_fee_per_gas),
            "balance" => Function::new_native_with_env(&store, env.clone(), balance),
            "network_size" => Function::new_native_with_env(&store, env.clone(), network_size),
            "identity_state" => Function::new_native_with_env(&store, env.clone(), identity_state),
            "identity" => Function::new_native_with_env(&store, env.clone(), identity),
            "send" => Function::new_native_with_env(&store, env.clone(), send),
            "caller" => Function::new_native_with_env(&store, env.clone(), caller),
            "origin_caller" => Function::new_native_with_env(&store, env.clone(), origin_caller),
            "create_call_function_promise" => Function::new_native_with_env(&store, env.clone(), create_call_function_promise),
            "create_transfer_promise" => Function::new_native_with_env(&store, env.clone(), create_transfer_promise),
            "promise_result" => Function::new_native_with_env(&store, env.clone(), crate::imports::promise_result),
            "promise_then" => Function::new_native_with_env(&store, env.clone(), promise_then),
            }
        };
        let module = match Module::new(&store, code) {
            Ok(v) => v,
            Err(err) => {
                return Err(VmError::custom(format!("compilation error: {}", err)));
            }
        };

        let instance = Instance::new(&module, &import_object)?;

        let wasmer_instance = Box::from(instance);

        let instance_ptr = NonNull::from(wasmer_instance.as_ref());
        env.set_wasmer_instance(Some(instance_ptr));
        Ok((env, module, wasmer_instance))
    }


    pub fn deploy<B: Backend + 'static>(api: B, code: Vec<u8>, args: protobuf::RepeatedField<Vec<u8>>, gas_limit: u64, gas_used: &mut u64) -> VmResult<ActionResult> {
        let (env, module, instance) = Self::build_env(api, code, None, gas_limit)?;

        let required_export = ["allocate", "deploy", "memory"];
        let module_info = module.info();
        for export in required_export {
            match module_info.exports.get(export) {
                Some(_) => continue,
                None => return Err(VmError::custom(format!("not found required export: {}", export)))
            }
        }

        let wasm_args = Self::prepare_arguments(&env.clone(), module.info(), &"deploy".to_string(), args)?;

        let res = match env.call_function("deploy", &wasm_args) {
            Ok(_) => {
                *gas_used = gas_limit.saturating_sub(env.get_gas_left());
                Ok(())
            }
            Err(err) => Err(err)
        };

        if res.is_err() {
            return Ok(Self::action_result_from_err(res.err().unwrap(), "deploy", *gas_used));
        }
        api.commit().0?;
        let mut res = Self::action_result_from_success("deploy", vec![], *gas_used);
        res.append_sub_action_results(Self::execute_promises(api, env));
        Ok(res)
    }


    pub fn apply_function_call<B: Backend + 'static>(api: B, recipient: Address, action: &FunctionCallAction, promise_result: Option<PromiseResult>, gas_used: &mut u64, is_callback: bool) -> VmResult<ActionResult> {
        let (code, used_gas) = api.contract_code(recipient);
        *gas_used = used_gas;
        let converted_args = convert_args(&action.args)?;
        Self::execute(api, code?, &action.method_name, converted_args, promise_result, action.gas_limit, gas_used, is_callback)
    }

    pub fn execute_promises<B: Backend + 'static>(api: B, env: Env<B>) -> Vec<ActionResult> {
        let promises = env.get_promises();
        println!("execute promises cnt={}", promises.len());
        if promises.is_empty() {
            return Vec::new();
        }
        let iter = promises.iter();
        let mut result: Vec<ActionResult> = Vec::with_capacity(iter.len());

        for p in iter {
            match &p.action {
                Action::FunctionCall(call) => {
                    let mut gas_used = 0;
                    println!("promise recipient = {:?}", p.receiver_Id);
                    let action_result = Self::apply_function_call(api, p.receiver_Id.to_vec(), call, None, &mut gas_used, false);

                    let promise_result = match action_result {
                        Ok(action_res) => {
                            result.push(action_res.clone());
                            if action_res.output_data.is_empty() {
                                Some(PromiseResult::Empty)
                            } else {
                                Some(PromiseResult::Value(action_res.output_data))
                            }
                        }
                        Err(err) => {
                            result.push(Self::action_result_from_err(err, call.method_name.as_str(), gas_used));
                            Some(PromiseResult::Failed)
                        }
                    };

                    if p.action_callback.is_some() {
                        match p.action_callback.clone().unwrap() {
                            Action::FunctionCall(call) => {
                                let mut gas_used = 0;
                                let action_result = Self::apply_function_call(api, api.contract().0.unwrap(), &call, promise_result, &mut gas_used, true);
                                result.push(action_result.unwrap_or_else(|err| Self::action_result_from_err(err, call.method_name.as_str(), gas_used)));
                            }
                            _ => unreachable!(),
                        };
                    }
                }
                Action::Transfer(t) => {
                    //todo : handle result
                    api.add_balance(p.receiver_Id.clone(), t.amount.to_vec());
                }
            };
        };
        result
    }


    pub fn execute<B: Backend + 'static>(api: B, code: Vec<u8>, method: &String, args: protobuf::RepeatedField<Vec<u8>>, promise_result: Option<PromiseResult>, gas_limit: u64, gas_used: &mut u64, is_callback: bool) -> VmResult<ActionResult> {
        let (env, module, instance) = Self::build_env(api, code, promise_result, gas_limit)?;
        Self::execute_with_env(env, module, api, method, args, gas_limit, gas_used, is_callback)
    }

    pub fn execute_with_env<B: Backend + 'static>(env: Env<B>, module: Module, api: B, method: &String, args: protobuf::RepeatedField<Vec<u8>>, gas_limit: u64, gas_used: &mut u64, is_callback: bool) -> VmResult<ActionResult> {
        if method == "deploy" {
            return Err(VmError::custom("direct call to deploy is forbidden'"));
        }
        if !is_callback && method.starts_with("_"){
            return Err(VmError::custom("direct call to promise callback is forbidden'"));
        }

        let wasm_args = Self::prepare_arguments(&env.clone(), module.info(), &method, args)?;
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
                    output_data = read_region(&env.memory(), ptr as u32, 1024).unwrap_or(vec![]);
                }
                *gas_used = gas_limit.saturating_sub(env.get_gas_left());
                Ok(())
            }
            Err(err) => {
                *gas_used = gas_limit.saturating_sub(env.get_gas_left());
                Err(err)
            }
        };
        if res.is_err() {
            return Ok(Self::action_result_from_err(res.err().unwrap(), method.as_str(), *gas_used));
        }
        api.commit().0?;
        let mut res = Self::action_result_from_success(method.as_str(), output_data, *gas_used);


        res.append_sub_action_results(Self::execute_promises(api, env));
        println!("action result={:?}", res);
        Ok(res)
    }

    fn action_result_from_err(err: VmError, method: &str, gas_used: u64) -> ActionResult {
        ActionResult {
            error: err.to_string(),
            success: false,
            gas_used: gas_used,
            input_action: Action::FunctionCall(FunctionCallAction {
                method_name: method.to_string(),
                gas_limit: 0,
                deposit: vec![],
                args: vec![],
            }),
            sub_action_results: vec![],
            output_data: vec![],
        }
    }
    fn action_result_from_success(method: &str, output_data: Vec<u8>, gas_used: u64) -> ActionResult {
        ActionResult {
            error: String::new(),
            success: true,
            gas_used: gas_used,
            input_action: Action::FunctionCall(FunctionCallAction {
                method_name: method.to_string(),
                gas_limit: 0,
                deposit: vec![],
                args: vec![],
            }),
            sub_action_results: vec![],
            output_data: output_data,
        }
    }
}
