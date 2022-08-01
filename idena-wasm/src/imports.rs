use protobuf::reflect::ProtobufValue;
use wasmer::{Array, Memory, WasmPtr};

use crate::backend::Backend;
use crate::environment::Env;
use crate::errors::VmError;
use crate::exports::gas_meter_t;
use crate::memory::{read_region, ref_to_u32, to_u32, VmResult, write_region};
use crate::types::{ActionResult, PromiseResult};
use crate::unwrap_or_return;

const MAX_STORAGE_KEY_SIZE: usize = 32;
const MAX_ADDRESS_SIZE: usize = 20;
const MAX_CODE_SIZE: usize = 1024 * 1024;
const MAX_IDNA_SIZE: usize = 32;
const MAX_STORAGE_VALUE_SIZE: usize = 128 * 1024;


pub fn process_gas_info<B: Backend>(
    env: &Env<B>,
    used_gas: u64,
) -> VmResult<()> {
    let gas_left = env.get_gas_left();
    println!("get gas left {}", gas_left);
    let gas_left = gas_left.saturating_sub(used_gas);

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
    println!("set_storage used_gas {}", gas);
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

pub fn block_seed<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    let gas_left = env.get_gas_left();

    env.backend.set_remaining_gas(gas_left);

    let (result, gas) = env.backend.block_seed();

    process_gas_info(env, gas)?;

    write_to_contract(env, &result?)
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
/*
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

pub fn call<B: Backend>(env: &Env<B>, addr: u32, method: u32, args: u32, amount: u32, gas_limit: u32) -> VmResult<ActionResult> {
    println!("{} {} {} {}", addr, method, args, gas_limit);

    let to = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;
    let method = read_region(&env.memory(), method, 1024)?;

    let amount = if amount > 0 { read_region(&env.memory(), amount, MAX_IDNA_SIZE)? } else { vec![] };

    let args = read_region(&env.memory(), args, 1024)?;

    let (res, _) = env.backend.call(to, &method, &args, &amount, gas_limit.into(), );

    Ok(res?)
}*/

pub fn caller<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.caller();
    process_gas_info(env, gas)?;
    let value = res?;
    write_to_contract(env, &value)
}

pub fn original_caller<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.original_caller();
    process_gas_info(env, gas)?;
    let value = res?;
    write_to_contract(env, &value)
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

    println!("wasm panicked: {}", msg);

    Err(VmError::wasm_err(msg))
}

pub fn promise_result<B: Backend>(env: &Env<B>, result: u32) -> VmResult<i32> {
    Ok(match &env.promise_result {
        Some(v) => {
            match v {
                PromiseResult::Empty => 1,
                PromiseResult::Failed => 0,
                PromiseResult::Value(data) => {
                    write_region(&env.memory(), result, &data);
                    2
                }
            }
        }
        None => 1
    })
}

pub fn create_call_function_promise<B: Backend>(env: &Env<B>, addr: u32, method: u32, args: u32, amount: u32, gas_limit: u32) -> VmResult<u32> {
    let to = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;
    let method = read_region(&env.memory(), method, 1024)?;
    let args = if args > 0 { read_region(&env.memory(), args, 1024)? } else { vec![] };
    let amountValue = if amount > 0 { read_region(&env.memory(), amount, MAX_IDNA_SIZE)? } else { vec![] };

    if !amountValue.is_empty() {
        let gas_left = env.get_gas_left();
        env.backend.set_remaining_gas(gas_left);
        let (res, gas) = env.backend.deduct_balance(amountValue.to_vec());
        process_gas_info(env, gas)?;
        res?;
    }
    let idx_res = env.create_function_call_promise(to, method, args, amountValue, gas_limit as u64);
    let idx = idx_res.0?;
    process_gas_info(env, gas_limit as u64)?;
    process_gas_info(env, idx_res.1)?;
    Ok(idx)
}

pub fn create_deploy_contract_promise<B: Backend>(env: &Env<B>, code: u32, args: u32, nonce: u32, amount: u32, gas_limit: u32) -> VmResult<u32> {
    println!("creating deploy promise codePtr = {:}, {:}, {:}, {:}, {:}", code, args, nonce, amount, gas_limit);
    let code = read_region(&env.memory(), code, MAX_CODE_SIZE)?;
    let args = if args > 0 { read_region(&env.memory(), args, 1024)? } else { vec![] };
    let nonce = if nonce > 0 { read_region(&env.memory(), nonce, 1024)? } else { vec![] };
    let amountValue = if amount > 0 { read_region(&env.memory(), amount, MAX_IDNA_SIZE)? } else { vec![] };

    if !amountValue.is_empty() {
        let gas_left = env.get_gas_left();
        env.backend.set_remaining_gas(gas_left);
        let (res, gas) = env.backend.deduct_balance(amountValue.to_vec());
        process_gas_info(env, gas)?;
        res?;
    }
    let idx_res = env.create_deploy_contract_promise(code, args, nonce, amountValue, gas_limit as u64);
    let idx = idx_res.0?;
    process_gas_info(env, gas_limit as u64)?;
    process_gas_info(env, idx_res.1)?;
    Ok(idx)
}

pub fn promise_then<B: Backend>(env: &Env<B>, promise_idx: u32, method: u32, args: u32, amount: u32, gas_limit: u32) -> VmResult<()> {
    let method = read_region(&env.memory(), method, 1024)?;
    let args = if args > 0 { read_region(&env.memory(), args, 1024)? } else { vec![] };
    let amount = if amount > 0 { read_region(&env.memory(), amount, MAX_IDNA_SIZE)? } else { vec![] };

    if !amount.is_empty() {
        let gas_left = env.get_gas_left();
        env.backend.set_remaining_gas(gas_left);
        let (res, gas) = env.backend.deduct_balance(amount.to_vec());
        process_gas_info(env, gas)?;
        res?;
    }

    env.promise_then(promise_idx as usize, method, args, amount, gas_limit as u64);
    process_gas_info(env, gas_limit as u64)
}

pub fn create_transfer_promise<B: Backend>(env: &Env<B>, addr: u32, amount: u32) -> VmResult<()> {
    let to = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;
    let amount = read_region(&env.memory(), amount, MAX_IDNA_SIZE)?;
    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.deduct_balance(amount.to_vec());
    process_gas_info(env, gas)?;
    res?;
    env.create_transfer_promise(to, amount);
    Ok(())
}

pub fn own_addr<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.own_addr();
    process_gas_info(env, gas)?;
    let addr = match res {
        Ok(v) => v,
        Err(err) => return Err(err.into())
    };
    write_to_contract(env, &addr)
}

pub fn contract_addr<B: Backend>(env: &Env<B>, code: u32, args: u32, nonce: u32) -> VmResult<u32> {
    let code = read_region(&env.memory(), code, MAX_CODE_SIZE)?;
    let args = if args > 0 { read_region(&env.memory(), args, 1024)? } else { vec![] };
    let nonce = if nonce > 0 { read_region(&env.memory(), nonce, 1024)? } else { vec![] };

    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.contract_addr(&code, &args, &nonce);
    process_gas_info(env, gas)?;
    let addr = match res {
        Ok(v) => v,
        Err(err) => return Err(err.into())
    };
    write_to_contract(env, &addr)
}

pub fn contract_addr_by_hash<B: Backend>(env: &Env<B>, hash: u32, args: u32, nonce: u32) -> VmResult<u32> {
    let hash = read_region(&env.memory(), hash, MAX_CODE_SIZE)?;
    let args = if args > 0 { read_region(&env.memory(), args, 1024)? } else { vec![] };
    let nonce = if nonce > 0 { read_region(&env.memory(), nonce, 1024)? } else { vec![] };

    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.contract_addr_by_hash(&hash, &args, &nonce);
    process_gas_info(env, gas)?;
    let addr = match res {
        Ok(v) => v,
        Err(err) => return Err(err.into())
    };
    write_to_contract(env, &addr)
}


pub fn own_code<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.own_code();
    process_gas_info(env, gas)?;
    let code = match res {
        Ok(v) => v,
        Err(err) => return Err(err.into())
    };
    write_to_contract(env, &code)
}

pub fn code_hash<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left);
    let (res, gas) = env.backend.code_hash();
    process_gas_info(env, gas)?;
    let hash = match res {
        Ok(v) => v,
        Err(err) => return Err(err.into())
    };
    write_to_contract(env, &hash)
}




