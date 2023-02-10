use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;

use thiserror::Error;

use crate::environment::Env;
use crate::types::{ActionResult, Address, IDNA};

#[derive(Error, Debug)]
pub enum BackendError {
    Custom { msg: String },
    OutOfGas,
}

impl Display for BackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendError::Custom { msg } => write!(f, "{}", msg),
            BackendError::OutOfGas => write!(f, "out_of_gas")
        }
    }
}

impl BackendError {
    pub fn new(msg: impl Into<String>) -> Self {
        BackendError::Custom {
            msg: msg.into()
        }
    }

    pub fn out_of_gas() -> Self {
        BackendError::OutOfGas
    }
}

pub type BackendResult<T> = (core::result::Result<T, BackendError>, u64);

pub trait Backend: Copy + Clone + Send {
    fn set_remaining_gas(&self, gasLimit: u64) -> BackendResult<()>;
    fn set_storage(&self, key: Vec<u8>, value: Vec<u8>) -> BackendResult<()>;
    fn get_storage(&self, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>>;
    fn remove_storage(&self, key: Vec<u8>) -> BackendResult<()>;
    fn block_timestamp(&self) -> BackendResult<i64>;
    fn block_number(&self) -> BackendResult<u64>;
    fn min_fee_per_gas(&self) -> BackendResult<IDNA>;
    fn balance(&self) -> BackendResult<IDNA>;
    fn block_seed(&self) -> BackendResult<Vec<u8>>;
    fn network_size(&self) -> BackendResult<u64>;
    fn identity_state(&self, addr: Address) -> BackendResult<u8>;
    fn pub_key(&self, addr: Address) -> BackendResult<Vec<u8>>;
    fn burn_all(&self) -> BackendResult<()>;
    fn read_contract_data(&self, addr: Address, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>>;
    fn epoch(&self) -> BackendResult<u16>;
    fn delegatee(&self, addr: Address) -> BackendResult<Option<Address>>;
    fn identity(&self, addr: Address) -> BackendResult<Option<Vec<u8>>>;
    fn call(&self, addr: Address, method: &[u8], args: &[u8], amount: &[u8], gas_limit: u64, invocation_ctx: &[u8]) -> BackendResult<ActionResult>;
    fn caller(&self) -> BackendResult<Vec<u8>>;
    fn original_caller(&self) -> BackendResult<Vec<u8>>;
    //fn commit(&self) -> BackendResult<()>;
    fn deduct_balance(&self, amount: IDNA) -> BackendResult<()>;
    fn add_balance(&self, to: Address, amount: IDNA) -> BackendResult<()>;
    fn own_addr(&self) -> BackendResult<Address>;
    fn contract_code(&self, contract: Address) -> BackendResult<Vec<u8>>;
    fn contract_addr(&self, code:  &[u8], args: &[u8], nonce: &[u8]) -> BackendResult<Address>;
    fn deploy(&self, code : &[u8], args: &[u8], nonce: &[u8], amount: &[u8], gas_limit: u64) -> BackendResult<ActionResult>;
    fn contract_addr_by_hash(&self, hash:  &[u8], args: &[u8], nonce: &[u8]) -> BackendResult<Address>;
    fn own_code(&self) -> BackendResult<Vec<u8>>;
    fn code_hash(&self) -> BackendResult<Vec<u8>>;
    fn event(&self, event_name : &[u8], args : &[u8]) -> BackendResult<()>;
    fn pay_amount(&self) -> BackendResult<IDNA>;
    fn block_header(&self, height : u64) -> BackendResult<Option<Vec<u8>>>;
    fn keccak256(&self, data: &[u8]) -> BackendResult<Vec<u8>>;
    fn global_state(&self) -> BackendResult<Vec<u8>>;
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
        println!("called set_remaining_gas");
        (Ok(()), 0)
    }

    fn set_storage(&self, key: Vec<u8>, value: Vec<u8>) -> BackendResult<()> {
        println!("called set_storage");
        (Ok(()), 0)
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

    fn min_fee_per_gas(&self) -> BackendResult<IDNA> {
        todo!()
    }

    fn balance(&self) -> BackendResult<IDNA> {
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

    fn read_contract_data(&self, addr: Address, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>> {
        todo!()
    }

    fn epoch(&self) -> BackendResult<u16> {
        todo!()
    }

    fn delegatee(&self, addr: Address) -> BackendResult<Option<Address>> {
        todo!()
    }

    fn identity(&self, addr: Address) -> BackendResult<Option<Vec<u8>>> {
        todo!()
    }

    fn call(&self, addr: Address, method: &[u8], args: &[u8], amount: &[u8], gas_limit: u64, invocation_ctx: &[u8]) -> BackendResult<ActionResult> {
        todo!()
    }

    fn caller(&self) -> BackendResult<Vec<u8>> {
        (Ok(vec![1, 2, 3]), 10)
    }
    fn original_caller(&self) -> BackendResult<Vec<u8>> {
        todo!()
    }

    fn deduct_balance(&self, amount: IDNA) -> BackendResult<()> {
        todo!()
    }

    fn add_balance(&self, to: Address, amount: IDNA) -> BackendResult<()> {
        todo!()
    }

    fn own_addr(&self) -> BackendResult<Address> {
        todo!()
    }

    fn contract_code(&self, contract: Address) -> BackendResult<Vec<u8>> {
        todo!()
    }

    fn contract_addr(&self, code: &[u8], args: &[u8], nonce: &[u8]) -> BackendResult<Address> {
        todo!()
    }

    fn deploy(&self, code: &[u8], args: &[u8], nonce: &[u8], amount: &[u8], gas_limit: u64) -> BackendResult<ActionResult> {
        todo!()
    }

    fn contract_addr_by_hash(&self, hash: &[u8], args: &[u8], nonce: &[u8]) -> BackendResult<Address> {
        todo!()
    }

    fn own_code(&self) -> BackendResult<Vec<u8>> {
        todo!()
    }

    fn code_hash(&self) -> BackendResult<Vec<u8>> {
        todo!()
    }

    fn event(&self, event_name: &[u8], args: &[u8]) -> BackendResult<()> {
        todo!()
    }

    fn pay_amount(&self) -> BackendResult<IDNA> {
        todo!()
    }

    fn block_header(&self, height: u64) -> BackendResult<Option<Vec<u8>>> {
        todo!()
    }

    fn keccak256(&self, data: &[u8]) -> BackendResult<Vec<u8>> {
        todo!()
    }

    fn global_state(&self) -> BackendResult<Vec<u8>> {
        todo!()
    }
}
