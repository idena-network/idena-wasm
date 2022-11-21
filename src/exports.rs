use std::{mem, slice, str};

use errno::{Errno, set_errno};
use protobuf::Message;

use crate::{check_go_result, proto};
use crate::args::convert_args;
use crate::backend::{Backend, BackendError, BackendResult};
use crate::costs::{BASE_CALL_COST, BASE_DEPLOY_COST};
use crate::environment::Env;
use crate::errors::VmError;
use crate::memory::{ByteSliceView, VmResult};
use crate::runner::VmRunner;
use crate::types::{Action, ActionResult, Address, IDNA, InvocationContext, PromiseResult};

#[repr(C)]
pub struct gas_meter_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct api_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GoApi_vtable {
    pub set_remaining_gas: extern "C" fn(
        *const api_t,
        u64,
    ) -> i32,
    pub set_storage: extern "C" fn(
        *const api_t,
        U8SliceView,
        U8SliceView,
        *mut u64,
    ) -> i32,
    pub get_storage: extern "C" fn(
        *const api_t,
        U8SliceView,
        *mut u64,
        *mut UnmanagedVector, // result output
    ) -> i32,
    pub remove_storage: extern "C" fn(
        *const api_t,
        U8SliceView,
        *mut u64,
    ) -> i32,
    pub block_number: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut u64, // result output
    ) -> i32,
    pub block_timestamp: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut i64, // result output
    ) -> i32,
    pub send: extern "C" fn(
        *const api_t,
        U8SliceView, // to
        U8SliceView, // amount
        *mut u64,
        *mut UnmanagedVector, // error
    ) -> i32,
    pub min_fee_per_gas: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub balance: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub block_seed: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub network_size: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut u64, // result
    ) -> i32,
    pub identity_state: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        *mut u64,
        *mut u8,
    ) -> i32,
    pub pub_key: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub burn_all: extern "C" fn(
        *const api_t,
        *mut u64,
    ) -> i32,
    pub epoch: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut u16,
    ) -> i32,
    pub contract_stake: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub move_to_stake: extern "C" fn(
        *const api_t,
        U8SliceView, // amount
        *mut u64,
        *mut UnmanagedVector, // output error
    ) -> i32,
    pub delegatee: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub identity: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub caller: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub original_caller: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub commit: extern "C" fn(
        *const api_t,
    ) -> i32,
    pub deduct_balance: extern "C" fn(
        *const api_t,
        U8SliceView, // amount
        *mut u64,
        *mut UnmanagedVector, // error
    ) -> i32,
    pub add_balance: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        U8SliceView, // amount
        *mut u64,
    ) -> i32,
    pub contract: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub contract_code: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub call: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        U8SliceView, // method
        U8SliceView, // args
        U8SliceView, // amount
        U8SliceView, // invocation ctx
        u64, // gas limit
        *mut u64,
        *mut UnmanagedVector, // action result
    ) -> i32,
    pub deploy: extern "C" fn(
        *const api_t,
        U8SliceView, // code
        U8SliceView, // args
        U8SliceView, // nonce
        U8SliceView, // amount
        u64, // gas limit
        *mut u64,
        *mut UnmanagedVector, // action result
    ) -> i32,
    pub contract_addr: extern "C" fn(
        *const api_t,
        U8SliceView, // code
        U8SliceView, // args,
        U8SliceView, // nonce,
        *mut u64,
        *mut UnmanagedVector, // addr
    ) -> i32,
    pub contract_addr_by_hash: extern "C" fn(
        *const api_t,
        U8SliceView, // hash
        U8SliceView, // args,
        U8SliceView, // nonce,
        *mut u64,
        *mut UnmanagedVector, // addr
    ) -> i32,
    pub own_code: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub code_hash: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub event: extern "C" fn(
        *const api_t,
        U8SliceView, // event_name
        U8SliceView, // args,
        *mut u64,
    ) -> i32,
    pub read_contract_data: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        U8SliceView, // key
        *mut u64,
        *mut UnmanagedVector, // data
    ) -> i32,
    pub pay_amount: extern "C" fn(
        *const api_t,
        *mut u64,
        *mut UnmanagedVector,
    ) -> i32,// amount
}

#[repr(C)]
pub struct U8SliceView {
    /// True if and only if this is None. If this is true, the other fields must be ignored.
    is_none: bool,
    ptr: *const u8,
    len: usize,
}

impl U8SliceView {
    pub fn new(source: Option<&[u8]>) -> Self {
        match source {
            Some(data) => Self {
                is_none: false,
                ptr: data.as_ptr(),
                len: data.len(),
            },
            None => Self {
                is_none: true,
                ptr: std::ptr::null::<u8>(),
                len: 0,
            },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnmanagedVector {
    /// True if and only if this is None. If this is true, the other fields must be ignored.
    is_none: bool,
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl UnmanagedVector {
    /// Consumes this optional vector for manual management.
    /// This is a zero-copy operation.
    pub fn new(source: Option<Vec<u8>>) -> Self {
        match source {
            Some(data) => {
                let (ptr, len, cap) = {
                    // Can be replaced with Vec::into_raw_parts when stable
                    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_raw_parts
                    let mut data = mem::ManuallyDrop::new(data);
                    (data.as_mut_ptr(), data.len(), data.capacity())
                };
                Self {
                    is_none: false,
                    ptr,
                    len,
                    cap,
                }
            }
            None => Self {
                is_none: true,
                ptr: std::ptr::null_mut::<u8>(),
                len: 0,
                cap: 0,
            },
        }
    }

    pub fn is_none(&self) -> bool {
        self.is_none
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Takes this UnmanagedVector and turns it into a regular, managed Rust vector.
    /// Calling this on two copies of UnmanagedVector leads to double free crashes.
    pub fn consume(self) -> Option<Vec<u8>> {
        if self.is_none {
            None
        } else {
            Some(unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) })
        }
    }
}

#[no_mangle]
pub extern "C" fn destroy_unmanaged_vector(v: UnmanagedVector) {
    let _ = v.consume();
}

impl Default for UnmanagedVector {
    fn default() -> Self {
        Self::new(None)
    }
}

#[no_mangle]
pub extern "C" fn new_unmanaged_vector(
    nil: bool,
    ptr: *const u8,
    length: usize,
) -> UnmanagedVector {
    if nil {
        UnmanagedVector::new(None)
    } else if length == 0 {
        UnmanagedVector::new(Some(Vec::new()))
    } else {
        let external_memory = unsafe { slice::from_raw_parts(ptr, length) };
        let copy = Vec::from(external_memory);
        UnmanagedVector::new(Some(copy))
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GoApi {
    pub state: *const api_t,
    pub gasMeter: *const gas_meter_t,
    pub vtable: GoApi_vtable,
}

pub struct apiWrapper {
    api: GoApi,
}

impl apiWrapper {
    fn new(api: GoApi) -> Self {
        apiWrapper {
            api
        }
    }
}

impl Copy for apiWrapper {}

impl Clone for apiWrapper {
    fn clone(&self) -> Self {
        apiWrapper {
            api: self.api.clone()
        }
    }
}

impl Backend for apiWrapper {
    fn set_remaining_gas(&self, gasLimit: u64) -> BackendResult<()> {
        let go_result = (self.api.vtable.set_remaining_gas)(self.api.state, gasLimit);
        check_go_result!(go_result, 0, "set_remaining_gas");
        (Ok(()), 0)
    }

    fn set_storage(&self, key: Vec<u8>, value: Vec<u8>) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.set_storage)(self.api.state, U8SliceView::new(Some(&key)), U8SliceView::new(Some(&value)), &mut used_gas as *mut u64);
        check_go_result!(go_result, used_gas,"set_storage");
        (Ok(()), used_gas)
    }

    fn get_storage(&self, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>> {
        let mut data = UnmanagedVector::default();
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.get_storage)(self.api.state, U8SliceView::new(Some(&key)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas,"get_storage");
        let result = data.consume();
        (Ok(result), used_gas)
    }

    fn remove_storage(&self, key: Vec<u8>) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.remove_storage)(self.api.state, U8SliceView::new(Some(&key)), &mut used_gas as *mut u64);
        check_go_result!(go_result, used_gas,"remove_storage");
        (Ok(()), used_gas)
    }

    fn block_timestamp(&self) -> BackendResult<i64> {
        let mut timestamp = 0_i64;
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.block_timestamp)(self.api.state, &mut used_gas as *mut u64, &mut timestamp as *mut i64);
        check_go_result!(go_result, used_gas,"block_timestamp");
        return (Ok(timestamp), used_gas);
    }

    fn block_number(&self) -> BackendResult<u64> {
        let mut height = 0_u64;
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.block_number)(self.api.state, &mut used_gas as *mut u64, &mut height as *mut u64);
        check_go_result!(go_result, used_gas,"block_number");
        return (Ok(height), used_gas);
    }

    fn min_fee_per_gas(&self) -> BackendResult<IDNA> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.min_fee_per_gas)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas,"min_fee_per_gas");
        let v = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(v), used_gas)
    }

    fn balance(&self, addr: Address) -> BackendResult<IDNA> {
        let mut used_gas = 0_u64;
        let mut balance = UnmanagedVector::default();
        let go_result = (self.api.vtable.balance)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut balance as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "balance");
        let amount = match balance.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(amount), used_gas)
    }

    fn block_seed(&self) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.block_seed)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "block_seed");
        let seed = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(seed), used_gas)
    }

    fn network_size(&self) -> BackendResult<u64> {
        let mut used_gas = 0_u64;
        let mut network_size = 0_u64;
        let go_result = (self.api.vtable.network_size)(self.api.state, &mut used_gas as *mut u64, &mut network_size as *mut u64);
        check_go_result!(go_result, used_gas, "network_size");
        (Ok(network_size), used_gas)
    }

    fn identity_state(&self, addr: Address) -> BackendResult<u8> {
        let mut used_gas = 0_u64;
        let mut state = 0_u8;
        let go_result = (self.api.vtable.identity_state)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut state as *mut u8);
        check_go_result!(go_result, used_gas, "identity_state");
        (Ok(state), used_gas)
    }

    fn pub_key(&self, addr: Address) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.pub_key)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "pub_key");
        let pub_key = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(pub_key), used_gas)
    }

    fn burn_all(&self) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.burn_all)(self.api.state, &mut used_gas as *mut u64);
        check_go_result!(go_result, used_gas, "burn_all");
        (Ok(()), used_gas)
    }

    fn read_contract_data(&self, addr: Address, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.read_contract_data)(self.api.state, U8SliceView::new(Some(&addr)), U8SliceView::new(Some(&key)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "read_contract_data");
        (Ok(data.consume()), used_gas)
    }

    fn epoch(&self) -> BackendResult<u16> {
        let mut used_gas = 0_u64;
        let mut epoch = 0_u16;
        let go_result = (self.api.vtable.epoch)(self.api.state, &mut used_gas as *mut u64, &mut epoch as *mut u16);
        check_go_result!(go_result, used_gas, "epoch");
        (Ok(epoch), used_gas)
    }

    fn delegatee(&self, addr: Address) -> BackendResult<Option<Address>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.delegatee)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "delegatee");
        (Ok(data.consume()), used_gas)
    }

    fn identity(&self, addr: Address) -> BackendResult<Option<Vec<u8>>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.identity)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "identity");
        (Ok(data.consume()), used_gas)
    }

    fn call(&self, addr: Address, method: &[u8], args: &[u8], amount: &[u8], gas_limit: u64, invocation_ctx: &[u8]) -> BackendResult<ActionResult> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        (self.api.vtable.call)(self.api.state, U8SliceView::new(Some(&addr)), U8SliceView::new(Some(method)),
                               U8SliceView::new(Some(args)), U8SliceView::new(Some(amount)), U8SliceView::new(Some(invocation_ctx)), gas_limit, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);

        let raw_data = match data.consume() {
            None => {
                return (Err(BackendError::new("action result bytes cannot be empty")), used_gas);
            }
            Some(v) => v
        };
        let res = match proto::models::ActionResult::parse_from_bytes(&raw_data) {
            Ok(m) => m,
            Err(e) => return (Err(BackendError::new("failed to parse function result")), used_gas)
        };
        (Ok(res.into()), used_gas)
    }

    fn caller(&self) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.caller)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "caller");
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn original_caller(&self) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.original_caller)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "original_caller");
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn commit(&self) -> BackendResult<()> {
        let go_result = (self.api.vtable.commit)(self.api.state);
        check_go_result!(go_result, 0, "commit");
        (Ok(()), 0)
    }

    fn deduct_balance(&self, amount: IDNA) -> BackendResult<()> {
        let mut err = UnmanagedVector::default();
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.deduct_balance)(self.api.state, U8SliceView::new(Some(&amount)), &mut used_gas as *mut u64, &mut err as *mut UnmanagedVector);
        if go_result != 0 {
            let err_data = match err.consume() {
                Some(v) => v,
                None => Vec::new()
            };
            let s = match String::from_utf8(err_data) {
                Ok(v) => v,
                Err(e) => "cannot parse backend error".to_string(),
            };
            return (Err(BackendError::new(format!("backend error: {}", s))), used_gas);
        }
        (Ok(()), used_gas)
    }

    fn add_balance(&self, to: Address, amount: IDNA) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.add_balance)(self.api.state, U8SliceView::new(Some(&to)), U8SliceView::new(Some(&amount)), &mut used_gas as *mut u64);
        check_go_result!(go_result, used_gas, "add_balance");
        (Ok(()), used_gas)
    }

    fn own_addr(&self) -> BackendResult<Address> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.contract)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "own_addr");
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn contract_code(&self, contract: Address) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.contract_code)(self.api.state, U8SliceView::new(Some(&contract)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas, "contract_code");
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn contract_addr(&self, code: &[u8], args: &[u8], nonce: &[u8]) -> BackendResult<Address> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.contract_addr)(self.api.state, U8SliceView::new(Some(&code)), U8SliceView::new(Some(&args)), U8SliceView::new(Some(&nonce)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas,"contract_addr");
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn deploy(&self, code: &[u8], args: &[u8], nonce: &[u8], amount: &[u8], gas_limit: u64) -> BackendResult<ActionResult> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        (self.api.vtable.deploy)(self.api.state, U8SliceView::new(Some(&code)), U8SliceView::new(Some(args)), U8SliceView::new(Some(nonce)),
                                 U8SliceView::new(Some(amount)), gas_limit, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        let raw_data = match data.consume() {
            None => {
                return (Err(BackendError::new("action result bytes cannot be empty")), used_gas);
            }
            Some(v) => v
        };
        let res = match proto::models::ActionResult::parse_from_bytes(&raw_data) {
            Ok(m) => m,
            Err(e) => return (Err(BackendError::new("failed to parse function result")), used_gas)
        };
        (Ok(res.into()), used_gas)
    }

    fn contract_addr_by_hash(&self, hash: &[u8], args: &[u8], nonce: &[u8]) -> BackendResult<Address> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.contract_addr_by_hash)(self.api.state, U8SliceView::new(Some(&hash)), U8SliceView::new(Some(&args)), U8SliceView::new(Some(&nonce)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas,"contract_addr_by_hash");
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn own_code(&self) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.own_code)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas,"own_code");
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn code_hash(&self) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.code_hash)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas,"code_hash");
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn event(&self, event_name: &[u8], args: &[u8]) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.event)(self.api.state, U8SliceView::new(Some(&event_name)), U8SliceView::new(Some(&args)), &mut used_gas as *mut u64);
        check_go_result!(go_result, used_gas,"emit event");
        (Ok(()), used_gas)
    }

    fn pay_amount(&self) -> BackendResult<IDNA> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.pay_amount)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        check_go_result!(go_result, used_gas,"pay_amount");
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }
}

unsafe impl Send for apiWrapper {}

unsafe impl Sync for apiWrapper {}


fn do_execute(api: GoApi, code: ByteSliceView,
              method_name: ByteSliceView,
              args: ByteSliceView,
              invocation_context: ByteSliceView,
              contract_addr: ByteSliceView,
              gas_limit: u64,
              gas_used: &mut u64) -> ActionResult {
    *gas_used = BASE_CALL_COST;

    let addr = contract_addr.read().unwrap_or(&[]);

    let data: Vec<u8> = match code.read() {
        Some(v) => v.to_vec(),
        None => return action_result_from_err(VmError::custom("code is required"), addr, gas_limit, *gas_used)
    };
    let arguments_bytes = args.read().unwrap_or(&[]);

    let method_bytes: Vec<u8> = match method_name.read() {
        Some(v) => v.into(),
        None => return action_result_from_err(VmError::custom("method is required"), addr, gas_limit, *gas_used)
    };

    let method = String::from_utf8_lossy(&method_bytes).to_string();

    if arguments_bytes.len() == 0 {
        return action_result_from_err(VmError::custom("invalid arguments format"), addr, gas_limit, *gas_used);
    }

    let args = match convert_args(arguments_bytes) {
        Ok(a) => a,
        Err(err) => return action_result_from_err(err, addr, gas_limit, *gas_used),
    };
    println!("execute code: code len={}, method={}, args={:?}, gas limit={}", data.len(), method, args, gas_limit);

    let mut ctx = InvocationContext::default();

    let ctx_bytes = invocation_context.read().unwrap_or(&[]);
    if ctx_bytes.len() > 0 {
        ctx = proto::models::InvocationContext::parse_from_bytes(ctx_bytes).unwrap_or_default().into()
    }

    VmRunner::execute(apiWrapper::new(api), addr, data, &method, arguments_bytes, gas_limit, gas_used, ctx)
}

fn action_result_from_err(err: VmError, contract_addr: &[u8], gas_limit: u64, gas_used: u64) -> ActionResult {
    ActionResult {
        error: err.to_string(),
        success: false,
        gas_used: gas_used,
        remaining_gas: gas_limit.saturating_sub(gas_used),
        input_action: Action::None,
        sub_action_results: vec![],
        output_data: vec![],
        contract: contract_addr.to_vec(),
    }
}


fn do_deploy(api: GoApi, code: ByteSliceView,
             args: ByteSliceView,
             contract_addr: ByteSliceView,
             gas_limit: u64,
             gas_used: &mut u64) -> ActionResult {
    *gas_used = BASE_DEPLOY_COST;
    let addr = contract_addr.read().unwrap_or(&[]);

    let data: Vec<u8> = match code.read() {
        Some(v) => v.to_vec(),
        None => return action_result_from_err(VmError::custom("code is required"), addr, gas_limit, *gas_used)
    };
    let arguments_bytes = args.read().unwrap_or(&[]);

    if arguments_bytes.len() == 0 {
        return action_result_from_err(VmError::custom("invalid arguments"), addr, gas_limit, *gas_used);
    }

    let args = match convert_args(arguments_bytes) {
        Ok(a) => a,
        Err(err) => return action_result_from_err(err, addr, gas_limit, *gas_used),
    };
    println!("deploy code: code len={}, args={:?}, gas limit={}", data.len(), args, gas_limit);
    VmRunner::deploy(apiWrapper::new(api), addr, data, arguments_bytes, gas_limit, gas_used)
}


#[no_mangle]
pub extern "C" fn execute(api: GoApi, code: ByteSliceView,
                          method_name: ByteSliceView,
                          args: ByteSliceView,
                          invocation_context: ByteSliceView,
                          contract_addr: ByteSliceView,
                          gas_limit: u64,
                          gas_used: &mut u64,
                          action_result: &mut UnmanagedVector,
                          err_msg: Option<&mut UnmanagedVector>) -> u8 {
    let res = do_execute(api, code, method_name, args, invocation_context, contract_addr, gas_limit, gas_used);
    let proto_action = Into::<crate::proto::models::ActionResult>::into(&res);
    *action_result = UnmanagedVector::new(Some(proto_action.write_to_bytes().unwrap_or(vec![])));
    0
}


#[no_mangle]
pub extern "C" fn deploy(api: GoApi, code: ByteSliceView,
                         args: ByteSliceView,
                         contract_addr: ByteSliceView,
                         gas_limit: u64,
                         gas_used: &mut u64,
                         action_result: &mut UnmanagedVector,
                         err_msg: Option<&mut UnmanagedVector>) -> u8 {
    let res = do_deploy(api, code, args, contract_addr, gas_limit, gas_used);
    let proto_action = Into::<crate::proto::models::ActionResult>::into(&res);
    *action_result = UnmanagedVector::new(Some(proto_action.write_to_bytes().unwrap_or(vec![])));
    0
}