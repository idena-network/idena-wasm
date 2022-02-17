use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

use crate::backend::BackendError;
use crate::errors::VmError::{Custom, OutOfGas};

#[derive(Error, Debug)]
pub enum VmError {
    #[error("Error calling the VM: {}", msg)]
    Custom {
        msg: String,
    },
    #[error("Out of gas")]
    OutOfGas,
}

impl VmError {
    pub fn custom(msg: impl Into<String>) -> Self {
        Custom {
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

impl From<BackendError> for VmError {
    fn from(original: BackendError) -> Self {
        VmError::custom(format!("backend error: {}", original))
    }
}