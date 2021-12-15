use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

use crate::environment::Env;

#[derive(Error, Debug)]
pub struct BackendError {
    msg: String,
}

impl Display for BackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl BackendError {
    pub fn new(msg: impl Into<String>) -> Self {
        BackendError {
            msg: msg.into()
        }
    }
}

pub type Address = [u8; 20];

pub type BackendResult<T> = (core::result::Result<T, BackendError>, u64);

pub trait Backend: Copy + Clone + Send {
    fn set_storage(&self, key: Vec<u8>, value: Vec<u8>) -> BackendResult<()>;
    fn get_storage(&self, env: &Env<Self>, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>>;
    fn remove_storage(&self, env: &Env<Self>, key: Vec<u8>) -> BackendResult<()>;
    fn block_timestamp(&self, env:&Env<Self>) -> BackendResult<i64>;
    fn block_number(&self, env: &Env<Self>) -> BackendResult<u64>;
    fn send(&self, env:&Env<Self>, to: Address ) -> BackendResult<()>;
}

pub struct MockBackend {}

impl MockBackend {
    pub fn new() -> Self {
        MockBackend {}
    }
}

impl Copy for MockBackend {}

impl Clone for MockBackend {
    fn clone(&self) -> Self {
        MockBackend {}
    }
}

impl Backend for MockBackend {
    fn set_storage(&self, key: Vec<u8>, value: Vec<u8>) -> BackendResult<()> {
        todo!()
    }

    fn get_storage(&self, env: &Env<Self>, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>> {
        todo!()
    }

    fn remove_storage(&self, env: &Env<Self>, key: Vec<u8>) -> BackendResult<()> {
        todo!()
    }

    fn block_timestamp(&self, env: &Env<Self>) -> BackendResult<i64> {
        todo!()
    }

    fn block_number(&self, env: &Env<Self>) -> BackendResult<u64> {
        todo!()
    }

    fn send(&self, env: &Env<Self>, to: Address) -> BackendResult<()> {
        todo!()
    }
}
