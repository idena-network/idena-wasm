use std::ptr::NonNull;
use std::sync::Arc;

use wasmer::{CompilerConfig, Exports, Function, ImportObject, imports, Instance, Module, Singlepass, Store};
use wasmer::wasmparser::Operator;
use wasmer_engine_universal::Universal;
use wasmer_middlewares::Metering;

use crate::backend::Backend;
use crate::environment::Env;
use crate::errors::VmError;
use crate::imports::*;
use crate::memory::VmResult;

pub struct VmRunner {}

impl VmRunner {
    pub fn execute<B: Backend + 'static>(api: B, code: Vec<u8>) -> VmResult<()> {
        let cost_function = |operator: &Operator| -> u64 {
            match operator {
                Operator::LocalGet { .. } | Operator::I32Const { .. } => 1,
                Operator::I32Add { .. } => 2,
                _ => 0,
            }
        };

        let metering = Arc::new(Metering::new(1000000000, cost_function));
        let mut compiler_config = Singlepass::default();
        compiler_config.push_middleware(metering);
        let store = Store::new(&Universal::new(compiler_config).engine());
        let env = Env::new(api);
        let import_object = imports! {
        "env" => {
            "set_storage" => Function::new_native_with_env(&store, env.clone(), set_storage),
            }
        };
        let module = match Module::new(&store, code) {
            Ok(v) => v,
            Err(err) => {
                println!("{:?}", err);
                return Err(VmError::new("compilation error"));
            }
        };

        let wasmer_instance = Box::from(Instance::new(&module, &import_object).unwrap());

        let instance_ptr = NonNull::from(wasmer_instance.as_ref());
        env.set_wasmer_instance(Some(instance_ptr));

        let main = wasmer_instance
            .exports
            .get_function("main").unwrap()
            .native::<(), u8>().unwrap();

        match main.call() {
            Ok(_) => Ok(()),
            Err(_) => Ok(())
        }
    }
}
