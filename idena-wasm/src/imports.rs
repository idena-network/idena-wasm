use wasmer::{Array, Memory, WasmPtr};

use crate::backend::Backend;
use crate::environment::Env;
use crate::errors::VmError;
use crate::exports::gas_meter_t;
use crate::memory::{read_region, ref_to_u32, to_u32, VmResult, write_region};

const MAX_STORAGE_KEY_SIZE: usize = 32;
const MAX_ADDRESS_SIZE: usize = 20;
const MAX_IDNA_SIZE: usize = 32;
const MAX_STORAGE_VALUE_SIZE: usize = 128 * 1024;


pub fn process_gas_info<B: Backend>(
    env: &Env<B>,
    used_gas: u64,
) -> VmResult<()> {
    let gas_left = env.get_gas_left();
    println!("get gas left {}", gas_left);
    let gas_left =  gas_left.saturating_sub(used_gas);

    // This tells wasmer how much more gas it can consume from this point in time.
    env.set_gas_left(gas_left);
    println!("set gas left {}", gas_left);
    if gas_left == 0 {
        Err(VmError::out_of_gas())
    } else {
        Ok(())
    }
}

/// Creates a Region in the contract, writes the given data to it and returns the memory location
pub fn write_to_contract<B: Backend>(
    env: &Env<B>,
    input: &[u8],
) -> VmResult<u32> {
    let out_size = to_u32(input.len())?;
    let result = env.call_function1("allocate", &[out_size.into()])?;
    let target_ptr = ref_to_u32(&result)?;
    if target_ptr == 0 {
        return Err(VmError::custom("target pointer is zero"));
    }
    write_region(&env.memory(), target_ptr, input)?;
    Ok(target_ptr)
}


pub fn set_storage<B: Backend>(env: &Env<B>, key: u32, value: u32) -> VmResult<()> {
    let key = read_region(&env.memory(), key, MAX_STORAGE_KEY_SIZE)?;
    let value = read_region(&env.memory(), value, MAX_STORAGE_VALUE_SIZE)?;

    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left).0?;

    let (result, gas) = env.backend.set_storage(key, value);

    process_gas_info(env, gas)?;

    result?;

    Ok(())
}

pub fn get_storage<B: Backend>(env: &Env<B>, key: u32) -> VmResult<u32> {
    let key = read_region(&env.memory(), key, MAX_STORAGE_KEY_SIZE)?;
    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.get_storage(key);

    process_gas_info(env, gas)?;
    let value = result?;

    let out_data = match value {
        Some(data) => data,
        None => return Ok(0),
    };
    write_to_contract(env, &out_data)
}

pub fn remove_storage<B: Backend>(env: &Env<B>, key: u32) -> VmResult<()> {
    let key = read_region(&env.memory(), key, MAX_STORAGE_KEY_SIZE)?;
    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.remove_storage(key);

    process_gas_info(env, gas)?;

    result?;

    Ok(())
}

pub fn block_timestamp<B: Backend>(env: &Env<B>) -> VmResult<i64> {
    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.block_timestamp();

    process_gas_info(env, gas)?;

    Ok(result?)
}

pub fn block_number<B: Backend>(env: &Env<B>) -> VmResult<u64> {
    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.block_number();

    process_gas_info(env, gas)?;

    Ok(result?)
}

pub fn min_fee_per_gas<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.min_fee_per_gas();

    process_gas_info(env, gas)?;

    write_to_contract(env, &result?)
}

pub fn balance<B: Backend>(env: &Env<B>, addr: u32) -> VmResult<u32> {
    let address = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;

    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.balance(address);

    process_gas_info(env, gas)?;

    write_to_contract(env, &result?)
}

pub fn network_size<B: Backend>(env: &Env<B>) -> VmResult<u64> {
    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.network_size();

    process_gas_info(env, gas)?;

    Ok(result?)
}


pub fn identity_state<B: Backend>(env: &Env<B>, addr: u32) -> VmResult<u8> {
    let address = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;

    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.identity_state(address);

    process_gas_info(env, gas)?;

    Ok(result?)
}

pub fn identity<B: Backend>(env: &Env<B>, addr: u32) -> VmResult<u32> {
    let address = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;

    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.identity(address);

    process_gas_info(env, gas)?;
    let value = result?;

    let out_data = match value {
        Some(data) => data,
        None => return Ok(0),
    };
    write_to_contract(env, &out_data)
}

pub fn send<B: Backend>(env: &Env<B>, to: u32, amount: u32) -> VmResult<i32> {
    let to = read_region(&env.memory(), to, MAX_ADDRESS_SIZE)?;

    let amount = read_region(&env.memory(), amount, MAX_IDNA_SIZE)?;

    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.send(to, amount);
    process_gas_info(env, gas)?;
    match res {
        Ok(_) => Ok(0),
        Err(_) => Ok(1),
    }
}

pub fn call<B: Backend>(env: &Env<B>, addr: u32, method: u32, args: u32, gas_limit: u32) -> VmResult<u8> {

    println!("{} {} {} {}", addr, method, args, gas_limit);

    let to = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;
    let method = read_region(&env.memory(), method, 1024)?;

    let args = read_region(&env.memory(), args, 1024)?;
    let gas_left = env.get_gas_left();
    if gas_left < gas_limit.into() {
        return Err(VmError::custom("not enough gas"));
    }

    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.call(to, method, args, gas_limit.into());
    println!("sub call used gas {}", gas);
    process_gas_info(env, gas)?;
    match res {
        Ok(_) => Ok(0),
        Err(_) => Ok(1),
    }
}


pub fn debug<B: Backend>(env: &Env<B>, ptr: u32) -> VmResult<()> {
    let message_data = read_region(&env.memory(), ptr, 1024)?;
    let msg = String::from_utf8_lossy(&message_data);
    println!("{}", msg);
    Ok(())
}

pub fn abort<B: Backend>(env: &Env<B>, msg: u32, filePtr: u32, line: u32, col: u32) {
    println!("called abort fn {} {} {} {}", msg, filePtr, line, col);
}

pub fn panic<B: Backend>(env: &Env<B>, msg: u32) -> VmResult<()> {
    let message_data = read_region(&env.memory(), msg, 1024)?;
    let msg = String::from_utf8_lossy(&message_data);
    Err(VmError::wasm_err(msg))
}






