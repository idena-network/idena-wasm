use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

#[derive(Error, Debug)]
pub struct VmError {
    msg: String,
}

impl Display for VmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl VmError {
    pub fn new(msg: impl Into<String>) -> Self {
        VmError {
            msg: msg.into()
        }
    }
}