use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};

use wasmer::{HostEnvInitError, Instance, Memory, WasmerEnv};

use crate::backend::Backend;
use crate::errors;
use crate::errors::VmError;
use crate::memory::VmResult;

pub struct Env<B: Backend> {
    pub backend: Option<B>,
    data: Arc<RwLock<ContextData>>,
}


impl<B: Backend> Env<B> {
    pub fn new(api: B) -> Self {
        Env {
            backend: Some(api),
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
            None => Err(VmError::new("uninitialized wasmer instance")),
        })
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
