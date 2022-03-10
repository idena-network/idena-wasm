use std::ptr::NonNull;
use std::sync::Arc;

use wasmer::{BaseTunables, CompilerConfig, Exportable, ExportIndex, Exports, Function, ImportObject, imports, Instance, Module, Pages, Singlepass, Store, Target, Val, Value};
use wasmer::wasmparser::Operator;
use wasmer_engine_universal::Universal;
use wasmer_middlewares::Metering;
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints};
use wasmer_types::ModuleInfo;

use crate::backend::Backend;
use crate::costs::*;
use crate::environment::Env;
use crate::errors::VmError;
use crate::gatekeeper::*;
use crate::imports::*;
use crate::limiting_tunables::LimitingTunables;
use crate::memory::VmResult;

pub struct VmRunner {}

impl VmRunner {
    fn prepare_arguments<B: Backend + 'static>(env: &Env<B>, info: &ModuleInfo, method: &str, args: protobuf::RepeatedField<Vec<u8>>) -> VmResult<Vec<Val>> {
        let mut params_cnt = 0;

        match info.exports.get(method) {
            Some(ExportIndex::Function(index)) => {
                let func = info.functions.get(index.clone()).unwrap();
                let sign = info.signatures.get(func.clone()).unwrap();
                if !sign.results().is_empty() {
                    return Err(VmError::custom("method cannot return value"));
                }
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

    fn build_env<B: Backend + 'static>(api: B, code: Vec<u8>, gas_limit: u64) -> VmResult<(Env<B>, Module, Box<Instance>)> {
        let metering = Arc::new(Metering::new(gas_limit, cost_function));
        let mut compiler_config = Singlepass::default();
        compiler_config.push_middleware(metering);
        compiler_config.push_middleware(Arc::new(Gatekeeper::default()));
        let base = BaseTunables::for_target(&Target::default());
        let store = Store::new_with_tunables(&Universal::new(compiler_config).engine(), LimitingTunables::new(base, Pages(100)));
        let env = Env::new(api);
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
            "call" => Function::new_native_with_env(&store, env.clone(), call),
            }
        };
        let module = match Module::new(&store, code) {
            Ok(v) => v,
            Err(err) => {
                return Err(VmError::custom(format!("compilation error: {}", err)));
            }
        };


        let wasmer_instance = Box::from(Instance::new(&module, &import_object).unwrap());

        let instance_ptr = NonNull::from(wasmer_instance.as_ref());
        env.set_wasmer_instance(Some(instance_ptr));
        Ok((env, module, wasmer_instance))
    }


    pub fn deploy<B: Backend + 'static>(api: B, code: Vec<u8>, args: protobuf::RepeatedField<Vec<u8>>, gas_limit: u64, gas_used: &mut u64) -> VmResult<()> {
        let (env, module, instance) = Self::build_env(api, code, gas_limit)?;

        let required_export = ["allocate", "deploy", "memory"];
        let module_info = module.info();
        for export in required_export {
            match module_info.exports.get(export) {
                Some(_) => continue,
                None => return Err(VmError::custom(format!("not found required export: {}", export)))
            }
        }

        let wasm_args = Self::prepare_arguments(&env.clone(), module.info(), "deploy", args)?;

        match env.call_function("deploy", &wasm_args) {
            Ok(_) => {
                *gas_used = gas_limit.saturating_sub(env.get_gas_left());
                Ok(())
            }
            Err(err) => Err(err)
        }
    }


    pub fn execute<B: Backend + 'static>(api: B, code: Vec<u8>, method: &str, args: protobuf::RepeatedField<Vec<u8>>, gas_limit: u64, gas_used: &mut u64) -> VmResult<()> {
        let (env, module, instance) = Self::build_env(api, code, gas_limit)?;

        let wasm_args = Self::prepare_arguments(&env.clone(), module.info(), method, args)?;

        match env.call_function(method, &wasm_args) {
            Ok(_) => {
                *gas_used = gas_limit.saturating_sub(env.get_gas_left());
                Ok(())
            }
            Err(err) => Err(err)
        }
    }
}
