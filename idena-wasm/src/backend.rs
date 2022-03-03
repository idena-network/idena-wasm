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

pub type Address = Vec<u8>;

pub type IDNA = Vec<u8>;

pub type BackendResult<T> = (core::result::Result<T, BackendError>, u64);

pub trait Backend: Copy + Clone + Send {
    fn set_remaining_gas(&self, gasLimit: u64) -> BackendResult<()>;
    fn set_storage(&self, key: Vec<u8>, value: Vec<u8>) -> BackendResult<()>;
    fn get_storage(&self, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>>;
    fn remove_storage(&self, key: Vec<u8>) -> BackendResult<()>;
    fn block_timestamp(&self) -> BackendResult<i64>;
    fn block_number(&self) -> BackendResult<u64>;
    fn send(&self, to: Address, amount: IDNA) -> BackendResult<()>;
    fn min_fee_per_gas(&self) -> BackendResult<IDNA>;
    fn balance(&self, addr: Address) -> BackendResult<IDNA>;
    fn block_seed(&self) -> BackendResult<Vec<u8>>;
    fn network_size(&self) -> BackendResult<u64>;
    fn identity_state(&self, addr: Address) -> BackendResult<u8>;
    fn pub_key(&self, addr: Address) -> BackendResult<Vec<u8>>;
    fn burn_all(&self) -> BackendResult<()>;
    fn read_contract_data(&self, addr: Address, key: Vec<u8>) -> BackendResult<Vec<u8>>;
    fn epoch(&self) -> BackendResult<u16>;
    fn contract_stake(&self, addr: Address) -> BackendResult<IDNA>;
    fn move_to_stake(&self, amount: IDNA) -> BackendResult<()>;
    fn delegatee(&self, addr: Address) -> BackendResult<Option<Address>>;
    fn identity(&self, addr: Address) -> BackendResult<Option<Vec<u8>>>;
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
    fn set_remaining_gas(&self, gasLimit: u64) -> BackendResult<()> {
        todo!()
    }

    fn set_storage(&self, key: Vec<u8>, value: Vec<u8>) -> BackendResult<()> {
        todo!()
    }

    fn get_storage(&self, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>> {
        todo!()
    }

    fn remove_storage(&self, key: Vec<u8>) -> BackendResult<()> {
        todo!()
    }

    fn block_timestamp(&self) -> BackendResult<i64> {
        todo!()
    }

    fn block_number(&self) -> BackendResult<u64> {
        todo!()
    }

    fn send(&self, to: Address, amount: Vec<u8>) -> BackendResult<()> {
        todo!()
    }

    fn min_fee_per_gas(&self) -> BackendResult<IDNA> {
        todo!()
    }

    fn balance(&self, addr: Address) -> BackendResult<IDNA> {
        todo!()
    }

    fn block_seed(&self) -> BackendResult<Vec<u8>> {
        todo!()
    }

    fn network_size(&self) -> BackendResult<u64> {
        todo!()
    }

    fn identity_state(&self, addr: Address) -> BackendResult<u8> {
        todo!()
    }

    fn pub_key(&self, addr: Address) -> BackendResult<Vec<u8>> {
        todo!()
    }

    fn burn_all(&self) -> BackendResult<()> {
        todo!()
    }

    fn read_contract_data(&self, addr: Address, key: Vec<u8>) -> BackendResult<Vec<u8>> {
        todo!()
    }

    fn epoch(&self) -> BackendResult<u16> {
        todo!()
    }

    fn contract_stake(&self, addr: Address) -> BackendResult<IDNA> {
        todo!()
    }

    fn move_to_stake(&self, amount: IDNA) -> BackendResult<()> {
        todo!()
    }

    fn delegatee(&self, addr: Address) -> BackendResult<Option<Address>> {
        todo!()
    }

    fn identity(&self, addr: Address) -> BackendResult<Option<Vec<u8>>> {
        todo!()
    }
}
