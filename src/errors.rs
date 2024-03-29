use std::fmt::{Debug};

use thiserror::Error;

use crate::backend::BackendError;
use crate::errors::VmError::{Custom, OutOfGas, WasmExecutionErr};

#[derive(Error, Debug)]
pub enum VmError {
    #[error("Error calling the VM: {}", msg)]
    Custom {
        msg: String,
    },
    #[error("Out of gas")]
    OutOfGas,
    #[error("Error in wasm module: {}", msg)]
    WasmExecutionErr {
        msg: String
    },
}

impl VmError {
    pub fn custom(msg: impl Into<String>) -> Self {
        Custom {
            msg: msg.into()
        }
    }

    pub fn wasm_err(msg: impl Into<String>) -> Self {
        WasmExecutionErr {
            msg: msg.into()
        }
    }

    pub fn out_of_gas() -> Self {
        OutOfGas {}
    }
}

impl From<wasmer::ExportError> for VmError {
    fn from(original: wasmer::ExportError) -> Self {
        VmError::custom(format!("Could not get export: {}", original))
    }
}

impl From<wasmer::InstantiationError> for VmError {
    fn from(original: wasmer::InstantiationError) -> Self {
        VmError::custom(format!("Failed to instantiate module: {}", original))
    }
}


impl From<BackendError> for VmError {
    fn from(original: BackendError) -> Self {
        VmError::custom(format!("backend error: {}", original))
    }
}