use std::{mem, slice};

use crate::backend::{Address, Backend, BackendError, BackendResult};
use crate::environment::Env;
use crate::memory::ByteSliceView;
use crate::runner::VmRunner;

#[repr(C)]
pub struct api_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GoApi_vtable {
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
        &mut u64, // result output
    ) -> i32,
    pub block_timestamp: extern "C" fn(
        *const api_t,
        *mut u64,
        &mut i64, // result output
    ) -> i32,
    pub send: extern "C" fn(
        *const api_t,
        U8SliceView,
        *mut u64,
        *mut UnmanagedVector, // error
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
    fn set_storage(&self, key: Vec<u8>, value: Vec<u8>) -> BackendResult<()> {
        let mut used_gas = 0_u64;
        let goResult = (self.api.vtable.set_storage)(self.api.state, U8SliceView::new(Some(&key)), U8SliceView::new(Some(&value)), &mut used_gas as *mut u64,);
        if goResult != 0 {
            return (Err(BackendError::new("backend error")), used_gas);
        }
        (Ok(()), used_gas)
    }

    fn get_storage(&self, env: &Env<Self>, key: Vec<u8>) -> BackendResult<Option<Vec<u8>>> {
        let mut data = UnmanagedVector::default();
        let mut used_gas = 0_u64;
        let goResult = (self.api.vtable.get_storage)(self.api.state, U8SliceView::new(Some(&key)), &mut used_gas as *mut u64, &mut data as *mut UnmanagedVector);
        if goResult != 0 {
           return (Err(BackendError::new("backend error")), used_gas);
        }
        let result = data.consume();
        (Ok(result), used_gas)
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

unsafe impl Send for apiWrapper {}

unsafe impl Sync for apiWrapper {}


#[no_mangle]
pub extern "C" fn execute(api: GoApi, code: ByteSliceView) -> i32 {
    let data: Vec<u8> = code.read().unwrap().into();
    VmRunner::execute(apiWrapper::new(api), data).unwrap();
    return 0;
}