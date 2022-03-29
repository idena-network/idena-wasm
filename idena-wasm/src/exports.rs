use std::{mem, slice, str};

use errno::{Errno, set_errno};
use protobuf::Message;

use crate::backend::{Address, Backend, BackendError, BackendResult, IDNA};
use crate::environment::Env;
use crate::errors::VmError;
use crate::memory::{ByteSliceView, VmResult};
use crate::proto;
use crate::runner::VmRunner;

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
    pub read_contract_data: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        U8SliceView, // key
        *mut u64,
        *mut UnmanagedVector, // result
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
    pub call: extern "C" fn(
        *const api_t,
        U8SliceView, // addr
        U8SliceView, // method
        U8SliceView, // args
        U8SliceView, // amount
        u64, // gas limit
        *mut u64,
    ) -> i32,
    pub caller: extern "C" fn (
        *const api_t,
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
    pub origin_caller: extern "C" fn (
        *const api_t,
        *mut u64,
        *mut UnmanagedVector, // result
    ) -> i32,
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
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), 0);
        }
        (Ok(()), 0)
    }

    fn set_storage(&self, key: Vec<u8>, value: Vec<u8>) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.set_storage)(self.api.state, U8SliceView::new(Some(&key)), U8SliceView::new(Some(&value)), &mut used_gas as *mut u64);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        println!("set storage used gas {}", used_gas);
        (Ok(()), used_gas)
    }

    fn get_storage(&self, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>> {
        let mut data = UnmanagedVector::default();
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.get_storage)(self.api.state, U8SliceView::new(Some(&key)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        let result = data.consume();
        (Ok(result), used_gas)
    }

    fn send(&self, to: Address, amount: Vec<u8>) -> BackendResult<()> {
        let mut err = UnmanagedVector::default();
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.send)(self.api.state, U8SliceView::new(Some(&to)), U8SliceView::new(Some(&amount)), &mut used_gas as *mut u64, &mut err as *mut UnmanagedVector);
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

    fn remove_storage(&self, key: Vec<u8>) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.remove_storage)(self.api.state, U8SliceView::new(Some(&key)), &mut used_gas as *mut u64);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        (Ok(()), used_gas)
    }

    fn block_timestamp(&self) -> BackendResult<i64> {
        let mut timestamp = 0_i64;
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.block_timestamp)(self.api.state, &mut used_gas as *mut u64, &mut timestamp as *mut i64);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        return (Ok(timestamp), used_gas);
    }

    fn block_number(&self) -> BackendResult<u64> {
        let mut height = 0_u64;
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.block_number)(self.api.state, &mut used_gas as *mut u64, &mut height as *mut u64);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        return (Ok(height), used_gas);
    }

    fn min_fee_per_gas(&self) -> BackendResult<IDNA> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.min_fee_per_gas)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }

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
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
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
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
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
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        (Ok(network_size), used_gas)
    }

    fn identity_state(&self, addr: Address) -> BackendResult<u8> {
        let mut used_gas = 0_u64;
        let mut state = 0_u8;
        let go_result = (self.api.vtable.identity_state)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut state as *mut u8);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        (Ok(state), used_gas)
    }

    fn pub_key(&self, addr: Address) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.pub_key)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        let pub_key = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(pub_key), used_gas)
    }

    fn burn_all(&self) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.burn_all)(self.api.state, &mut used_gas as *mut u64);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        (Ok(()), used_gas)
    }

    fn read_contract_data(&self, addr: Address, key: Vec<u8>) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.read_contract_data)(self.api.state, U8SliceView::new(Some(&addr)), U8SliceView::new(Some(&key)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn epoch(&self) -> BackendResult<u16> {
        let mut used_gas = 0_u64;
        let mut epoch = 0_u16;
        let go_result = (self.api.vtable.epoch)(self.api.state, &mut used_gas as *mut u64, &mut epoch as *mut u16);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        (Ok(epoch), used_gas)
    }

    fn contract_stake(&self, addr: Address) -> BackendResult<IDNA> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.contract_stake)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        let stake = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(stake), used_gas)
    }

    fn move_to_stake(&self, amount: IDNA) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let mut err_data = UnmanagedVector::default();
        let go_result = (self.api.vtable.move_to_stake)(self.api.state, U8SliceView::new(Some(&amount)), &mut used_gas as *mut u64, &mut err_data as *mut UnmanagedVector);
        if go_result != 0 {
            let err_data = match err_data.consume() {
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

    fn delegatee(&self, addr: Address) -> BackendResult<Option<Address>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.delegatee)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        (Ok(data.consume()), used_gas)
    }

    fn identity(&self, addr: Address) -> BackendResult<Option<Vec<u8>>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.identity)(self.api.state, U8SliceView::new(Some(&addr)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        (Ok(data.consume()), used_gas)
    }

    fn call(&self, addr: Address, method: Vec<u8>, args: Vec<u8>, amount: Vec<u8>, gas_limit: u64) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let go_result = (self.api.vtable.call)(self.api.state, U8SliceView::new(Some(&addr)), U8SliceView::new(Some(&method)),
                                               U8SliceView::new(Some(&args)), U8SliceView::new(Some(&amount)), gas_limit, &mut used_gas as *mut u64);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        (Ok(()), used_gas)
    }

    fn caller(&self) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.caller)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }

    fn origin_caller(&self) -> BackendResult<Vec<u8>> {
        let mut used_gas = 0_u64;
        let mut data = UnmanagedVector::default();
        let go_result = (self.api.vtable.origin_caller)(self.api.state, &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if go_result != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        let d = match data.consume() {
            Some(v) => v,
            None => Vec::new()
        };
        (Ok(d), used_gas)
    }
}

unsafe impl Send for apiWrapper {}

unsafe impl Sync for apiWrapper {}


const ARGS_PROTOBUF_FORMAT: u8 = 0x1;
const ARGS_PLAIN_FORMAT: u8 = 0x0;

fn do_execute(api: GoApi, code: ByteSliceView,
              method_name: ByteSliceView,
              args: ByteSliceView,
              gas_limit: u64,
              gas_used: &mut u64) -> VmResult<()> {
    let data: Vec<u8> = code.read().ok_or_else(|| VmError::custom("code is required"))?.into();
    let arguments_bytes = args.read().unwrap_or(&[]);

    let method_bytes: Vec<u8> = method_name.read().ok_or_else(|| VmError::custom("method is required"))?.into();
    let method = String::from_utf8_lossy(&method_bytes).to_string();


    if arguments_bytes.len() == 0 {
        return Err(VmError::custom("invalid arguments"));
    }


    let args: protobuf::RepeatedField<Vec<u8>>;

    match arguments_bytes[0] {
        ARGS_PROTOBUF_FORMAT => {
            args = proto::models::ProtoCallContractArgs::parse_from_bytes(&arguments_bytes[1..])
                .or(Err(VmError::custom("failed to parse arguments")))?.args;
        }
        ARGS_PLAIN_FORMAT => { args = protobuf::RepeatedField::from_vec(vec![arguments_bytes[1..].to_vec()]); }
        _ => return Err(VmError::custom("unknown format of args"))
    }

    println!("execute code: code len={}, method={}, args={:?}, gas limit={}", data.len(), method, args, gas_limit);

    VmRunner::execute(apiWrapper::new(api), data, method.as_str(), args, gas_limit, gas_used)
}


fn do_deploy(api: GoApi, code: ByteSliceView,
              args: ByteSliceView,
              gas_limit: u64,
              gas_used: &mut u64) -> VmResult<()> {
    let data: Vec<u8> = code.read().ok_or_else(|| VmError::custom("code is required"))?.into();
    let arguments_bytes = args.read().unwrap_or(&[]);

    if arguments_bytes.len() == 0 {
        return Err(VmError::custom("invalid arguments"));
    }


    let args: protobuf::RepeatedField<Vec<u8>>;

    match arguments_bytes[0] {
        ARGS_PROTOBUF_FORMAT => {
            args = proto::models::ProtoCallContractArgs::parse_from_bytes(&arguments_bytes[1..])
                .or(Err(VmError::custom("failed to parse arguments")))?.args;
        }
        ARGS_PLAIN_FORMAT => { args = protobuf::RepeatedField::from_vec(vec![arguments_bytes[1..].to_vec()]); }
        _ => return Err(VmError::custom("unknown format of args"))
    }

    println!("deploy code: code len={}, args={:?}, gas limit={}", data.len(), args, gas_limit);

    VmRunner::deploy(apiWrapper::new(api), data, args, gas_limit, gas_used)
}


#[no_mangle]
pub extern "C" fn execute(api: GoApi, code: ByteSliceView,
                          method_name: ByteSliceView,
                          args: ByteSliceView,
                          gas_limit: u64,
                          gas_used: &mut u64,
                          err_msg: Option<&mut UnmanagedVector>) -> u8 {
    match do_execute(api, code, method_name, args, gas_limit, gas_used) {
        Ok(_) => 0,
        Err(err) => {
            *gas_used = gas_limit;

            if let Some(err_msg) = err_msg {
                if err_msg.is_some() {
                    panic!(
                        "There is an old error message in the given pointer that has not been \
                cleaned up. Error message pointers should not be reused for multiple calls."
                    )
                }
                *err_msg = UnmanagedVector::new(Some(err.to_string().into()));
            } else {
                // The caller provided a nil pointer for the error message.
                // That's not nice but we can live with it.
            }
            match err {
                VmError::Custom { .. } => 1,
                VmError::OutOfGas => 2,
                VmError::WasmExecutionErr { .. } => 3
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn deploy(api: GoApi, code: ByteSliceView,
                         args: ByteSliceView,
                         gas_limit: u64,
                         gas_used: &mut u64,
                         err_msg: Option<&mut UnmanagedVector>) -> u8 {
    match do_deploy(api, code, args, gas_limit, gas_used) {
        Ok(_) => 0,
        Err(err) => {
            *gas_used = gas_limit;

            if let Some(err_msg) = err_msg {
                if err_msg.is_some() {
                    panic!(
                        "There is an old error message in the given pointer that has not been \
                cleaned up. Error message pointers should not be reused for multiple calls."
                    )
                }
                *err_msg = UnmanagedVector::new(Some(err.to_string().into()));
            } else {
                // The caller provided a nil pointer for the error message.
                // That's not nice but we can live with it.
            }
            match err {
                VmError::Custom { .. } => 1,
                VmError::OutOfGas => 2,
                VmError::WasmExecutionErr { .. } => 3
            }
        }
    }
}