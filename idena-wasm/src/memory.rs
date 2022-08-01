use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::slice;

use wasmer::{Array, ValueType, WasmPtr};

use crate::errors::VmError;

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
struct Region {
    /// The beginning of the region expressed as bytes from the beginning of the linear memory
    pub offset: u32,
    /// The number of bytes available in this region
    pub capacity: u32,
    /// The number of bytes used in this region
    pub length: u32,
}

pub type CommunicationResult<T> = core::result::Result<T, VmError>;
pub type RegionValidationResult<T> = core::result::Result<T, VmError>;
pub type VmResult<T> = core::result::Result<T, VmError>;


unsafe impl ValueType for Region {}

/// Safely converts input of type &T to u32.
/// Errors with a cosmwasm_vm::errors::VmError::ConversionErr if conversion cannot be done.
pub fn ref_to_u32<T: TryInto<u32> + ToString + Clone>(input: &T) -> VmResult<u32> {
    input.clone().try_into().map_err(|_| {
        VmError::custom(format!("Couldn't convert from {} to {}. Input: {}", type_name::<T>(), type_name::<u32>(), input.to_string()))
    })
}

pub fn to_u32<T: std::convert::TryInto<u32> + ToString + Copy>(input: T) -> VmResult<u32> {
    input.try_into().map_err(|_| {
        VmError::custom("conversion err")
    })
}

pub fn read_region(memory: &wasmer::Memory, ptr: u32, max_length: usize) -> VmResult<Vec<u8>> {
    println!("try to read region {:}", ptr);
    let region = get_region(memory, ptr)?;

    if region.length > to_u32(max_length)? {
        return Err(
            VmError::custom(format!("region_length_too_big: ptr={} expected max = {}, actual={}",ptr, max_length, region.length))
        );
    }

    match WasmPtr::<u8, Array>::new(region.offset).deref(memory, 0, region.length) {
        Some(cells) => {
            // In case you want to do some premature optimization, this shows how to cast a `&'mut [Cell<u8>]` to `&mut [u8]`:
            // https://github.com/wasmerio/wasmer/blob/0.13.1/lib/wasi/src/syscalls/mod.rs#L79-L81
            let len = region.length as usize;
            let mut result = vec![0u8; len];
            for i in 0..len {
                result[i] = cells[i].get();
            }
            Ok(result)
        }
        None => Err(VmError::custom(format!(
            "Tried to access memory of region {:?} in wasm memory of size {} bytes. This typically happens when the given Region pointer does not point to a proper Region struct.",
            region,
            memory.size().bytes().0
        ))),
    }
}


/// maybe_read_region is like read_region, but gracefully handles null pointer (0) by returning None
/// meant to be used where the argument is optional (like scan)
pub fn maybe_read_region(
    memory: &wasmer::Memory,
    ptr: u32,
    max_length: usize,
) -> VmResult<Option<Vec<u8>>> {
    if ptr == 0 {
        Ok(None)
    } else {
        read_region(memory, ptr, max_length).map(Some)
    }
}


/// A prepared and sufficiently large memory Region is expected at ptr that points to pre-allocated memory.
///
/// Returns number of bytes written on success.
pub fn write_region(memory: &wasmer::Memory, ptr: u32, data: &[u8]) -> VmResult<()> {
    let mut region = get_region(memory, ptr)?;

    let region_capacity = region.capacity as usize;
    if data.len() > region_capacity {
        return Err(VmError::custom("region_too_small"));
    }
    match WasmPtr::<u8, Array>::new(region.offset).deref(memory, 0, region.capacity) {
        Some(cells) => {
            // In case you want to do some premature optimization, this shows how to cast a `&'mut [Cell<u8>]` to `&mut [u8]`:
            // https://github.com/wasmerio/wasmer/blob/0.13.1/lib/wasi/src/syscalls/mod.rs#L79-L81
            for i in 0..data.len() {
                cells[i].set(data[i])
            }
            region.length = data.len() as u32;
            set_region(memory, ptr, region)?;
            Ok(())
        }
        None => Err(VmError::custom(format!(
            "Tried to access memory of region {:?} in wasm memory of size {} bytes. This typically happens when the given Region pointer does not point to a proper Region struct.",
            region,
            memory.size().bytes().0
        ))),
    }
}

/// Reads in a Region at ptr in wasm memory and returns a copy of it
fn get_region(memory: &wasmer::Memory, ptr: u32) -> CommunicationResult<Region> {
    let wptr = WasmPtr::<Region>::new(ptr);
    match wptr.deref(memory) {
        Some(cell) => {
            let region = cell.get();
            validate_region(&region)?;
            Ok(region)
        }
        None => Err(VmError::custom("Could not dereference this pointer to a Region"))
    }
}

/// Performs plausibility checks in the given Region. Regions are always created by the
/// contract and this can be used to detect problems in the standard library of the contract.
fn validate_region(region: &Region) -> RegionValidationResult<()> {
    if region.offset == 0 {
        return Err(VmError::custom("zero offset"));
    }
    if region.length > region.capacity {
        return Err(VmError::custom("length > capacity"));
    }
    if region.capacity > (u32::MAX - region.offset) {
        return Err(VmError::custom("out of range"));
    }
    Ok(())
}

/// Overrides a Region at ptr in wasm memory with data
fn set_region(memory: &wasmer::Memory, ptr: u32, data: Region) -> CommunicationResult<()> {
    let wptr = WasmPtr::<Region>::new(ptr);

    match wptr.deref(memory) {
        Some(cell) => {
            cell.set(data);
            Ok(())
        }
        None => Err(VmError::custom(
            "Could not dereference this pointer to a Region"
        )),
    }
}

#[repr(C)]
pub struct ByteSliceView {
    /// True if and only if the byte slice is nil in Go. If this is true, the other fields must be ignored.
    is_nil: bool,
    ptr: *const u8,
    len: usize,
}

impl ByteSliceView {
    /// ByteSliceViews are only constructed in Go. This constructor is a way to mimic the behaviour
    /// when testing FFI calls from Rust. It must not be used in production code.
    #[cfg(test)]
    pub fn new(source: &[u8]) -> Self {
        Self {
            is_nil: false,
            ptr: source.as_ptr(),
            len: source.len(),
        }
    }

    /// ByteSliceViews are only constructed in Go. This constructor is a way to mimic the behaviour
    /// when testing FFI calls from Rust. It must not be used in production code.
    #[cfg(test)]
    pub fn nil() -> Self {
        Self {
            is_nil: true,
            ptr: std::ptr::null::<u8>(),
            len: 0,
        }
    }

    /// Provides a reference to the included data to be parsed or copied elsewhere
    /// This is safe as long as the `ByteSliceView` is constructed correctly.
    pub fn read(&self) -> Option<&[u8]> {
        if self.is_nil {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(self.ptr, self.len) })
        }
    }

    /// Creates an owned copy that can safely be stored and mutated.
    #[allow(dead_code)]
    pub fn to_owned(&self) -> Option<Vec<u8>> {
        self.read().map(|slice| slice.to_owned())
    }
}