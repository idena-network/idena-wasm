use crate::backend::Backend;
use crate::costs::BASE_BYTES_TO_HEX_COST;
use crate::environment::Env;
use crate::errors::VmError;
use crate::memory::{read_region, read_u32, read_utf16_string, ref_to_u32, to_u32, VmResult, write_region};
use crate::types::{GetIdentityAction, PromiseResult, ReadContractDataAction, ReadShardedDataAction};

const MAX_STORAGE_KEY_SIZE: usize = 128 * 1024;
const MAX_ADDRESS_SIZE: usize = 20;
const MAX_CODE_SIZE: usize = 1024 * 1024;
const MAX_IDNA_SIZE: usize = 32;
const MAX_STORAGE_VALUE_SIZE: usize = 128 * 1024;
const MAX_STRING_SIZE: usize = 4 * 1024;
const MAX_ARGS_SIZE: usize = 10 * 1024;
pub const MAX_RETURN_VALUE_SIZE: usize = 64 * 1024;

pub fn process_gas_info<B: Backend>(
    env: &Env<B>,
    used_gas: u64,
) -> VmResult<()> {
    let gas_left = env.get_gas_left();
    let gas_left = gas_left.saturating_sub(used_gas);

    // This tells wasmer how much more gas it can consume from this point in time.
    env.set_gas_left(gas_left);
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

    set_left_gas_to_backend(env)?;

    let (result, gas) = env.backend.set_storage(key, value);
    process_gas_info(env, gas)?;

    result?;

    Ok(())
}

pub fn get_storage<B: Backend>(env: &Env<B>, key: u32) -> VmResult<u32> {
    let key = read_region(&env.memory(), key, MAX_STORAGE_KEY_SIZE)?;
    set_left_gas_to_backend(env)?;

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
    set_left_gas_to_backend(env)?;

    let (result, gas) = env.backend.remove_storage(key);

    process_gas_info(env, gas)?;

    result?;

    Ok(())
}

pub fn block_timestamp<B: Backend>(env: &Env<B>) -> VmResult<i64> {
    set_left_gas_to_backend(env)?;

    let (result, gas) = env.backend.block_timestamp();

    process_gas_info(env, gas)?;

    Ok(result?)
}

pub fn block_number<B: Backend>(env: &Env<B>) -> VmResult<u64> {
    set_left_gas_to_backend(env)?;

    let (result, gas) = env.backend.block_number();

    process_gas_info(env, gas)?;

    Ok(result?)
}

pub fn block_seed<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;

    let (result, gas) = env.backend.block_seed();

    process_gas_info(env, gas)?;

    write_to_contract(env, &result?)
}

pub fn min_fee_per_gas<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;

    let (result, gas) = env.backend.min_fee_per_gas();

    process_gas_info(env, gas)?;

    write_to_contract(env, &result?)
}

pub fn balance<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;

    let (result, gas) = env.backend.balance();

    process_gas_info(env, gas)?;

    write_to_contract(env, &result?)
}

pub fn network_size<B: Backend>(env: &Env<B>) -> VmResult<u64> {
    set_left_gas_to_backend(env)?;

    let (result, gas) = env.backend.network_size();

    process_gas_info(env, gas)?;

    Ok(result?)
}

pub fn event<B: Backend>(env: &Env<B>, event_name: u32, args: u32) -> VmResult<()> {
    let event_name = read_region(&env.memory(), event_name, MAX_STRING_SIZE)?;

    let args = if args > 0 { read_region(&env.memory(), args, MAX_ARGS_SIZE)? } else { vec![] };
    set_left_gas_to_backend(env)?;

    let (result, gas) = env.backend.event(&event_name, &args);

    process_gas_info(env, gas)?;
    result?;
    Ok(())
}

pub fn epoch<B: Backend>(env: &Env<B>) -> VmResult<i32> {
    set_left_gas_to_backend(env)?;
    let (result, gas) = env.backend.epoch();
    process_gas_info(env, gas)?;
    Ok(result? as i32)
}

pub fn pay_amount<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;
    let (result, gas) = env.backend.pay_amount();
    process_gas_info(env, gas)?;
    write_to_contract(env, &result?)
}

pub fn caller<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;
    let (res, gas) = env.backend.caller();
    process_gas_info(env, gas)?;
    let value = res?;
    write_to_contract(env, &value)
}

pub fn original_caller<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;
    let (res, gas) = env.backend.original_caller();
    process_gas_info(env, gas)?;
    let value = res?;
    write_to_contract(env, &value)
}


pub fn debug<B: Backend>(env: &Env<B>, ptr: u32) -> VmResult<()> {
    let message_data = read_region(&env.memory(), ptr, MAX_STRING_SIZE)?;
    let msg = String::from_utf8_lossy(&message_data);
    println!("{}", msg);
    Ok(())
}

pub fn abort<B: Backend>(env: &Env<B>, msg: u32, file_ptr: u32, line: u32, col: u32) -> VmResult<()> {
    if msg >= 4 && file_ptr >= 4 {
        let mem = env.memory();
        let msg_len = read_u32(&mem, (msg - 4) as u32);
        let file_len = read_u32(&mem, (file_ptr - 4) as u32);
        let str = read_utf16_string(&mem, msg, msg_len?)?;
        let file = read_utf16_string(&mem, file_ptr, file_len?)?;
        let message = format!("{}, filename: \"{}\" line: {} col: {}", str, file, line, col);
        println!("called abort fn: {} ", message);
        return Err(VmError::wasm_err(message));
    }
    Err(VmError::custom("bad utf16 format"))
}

pub fn panic<B: Backend>(env: &Env<B>, msg: u32) -> VmResult<()> {
    let message_data = read_region(&env.memory(), msg, MAX_STRING_SIZE)?;
    let msg = String::from_utf8_lossy(&message_data);

    println!("wasm panicked: {}", msg);

    Err(VmError::wasm_err(msg))
}

pub fn promise_result<B: Backend>(env: &Env<B>, status: u32) -> VmResult<u32> {
    Ok(match &env.promise_result {
        Some(v) => {
            match v {
                PromiseResult::Empty => {
                    write_region(&env.memory(), status, &[1])?;
                    0
                }
                PromiseResult::Failed => {
                    write_region(&env.memory(), status, &[0])?;
                    0
                }
                PromiseResult::Value(data) => {
                    write_region(&env.memory(), status, &[2])?;
                    write_to_contract(&env, data)?
                }
            }
        }
        None => {
            write_region(&env.memory(), status, &[1])?;
            0
        }
    })
}

pub fn create_call_function_promise<B: Backend>(env: &Env<B>, addr: u32, method: u32, args: u32, amount: u32, gas_limit: u32) -> VmResult<u32> {
    let to = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;
    let method = read_region(&env.memory(), method, MAX_STRING_SIZE)?;
    let args = if args > 0 { read_region(&env.memory(), args, MAX_ARGS_SIZE)? } else { vec![] };
    let amount_value = if amount > 0 { read_region(&env.memory(), amount, MAX_IDNA_SIZE)? } else { vec![] };

    deduct_balance_if_needed(env, &amount_value)?;

    let idx_res = env.create_function_call_promise(to, method, args, amount_value, gas_limit as u64);
    let idx = idx_res.0?;
    process_gas_info(env, gas_limit as u64)?;
    process_gas_info(env, idx_res.1)?;
    Ok(idx)
}

pub fn create_deploy_contract_promise<B: Backend>(env: &Env<B>, code: u32, args: u32, nonce: u32, amount: u32, gas_limit: u32) -> VmResult<u32> {
    let code = read_region(&env.memory(), code, MAX_CODE_SIZE)?;
    let args = if args > 0 { read_region(&env.memory(), args, MAX_ARGS_SIZE)? } else { vec![] };
    let nonce = if nonce > 0 { read_region(&env.memory(), nonce, MAX_STRING_SIZE)? } else { vec![] };
    let amount_value = if amount > 0 { read_region(&env.memory(), amount, MAX_IDNA_SIZE)? } else { vec![] };

    deduct_balance_if_needed(env, &amount_value)?;
    let idx_res = env.create_deploy_contract_promise(code, args, nonce, amount_value, gas_limit as u64);
    let idx = idx_res.0?;
    process_gas_info(env, gas_limit as u64)?;
    process_gas_info(env, idx_res.1)?;
    Ok(idx)
}

fn deduct_balance_if_needed<B: Backend>(env: &Env<B>, amount_value: &Vec<u8>) -> VmResult<()> {
    if !amount_value.is_empty() {
        set_left_gas_to_backend(env)?;
        let (res, gas) = env.backend.deduct_balance(amount_value.to_vec());
        process_gas_info(env, gas)?;
        res?;
    }
    Ok(())
}

pub fn promise_then<B: Backend>(env: &Env<B>, promise_idx: u32, method: u32, args: u32, amount: u32, gas_limit: u32) -> VmResult<()> {
    let method = read_region(&env.memory(), method, MAX_STRING_SIZE)?;
    let args = if args > 0 { read_region(&env.memory(), args, MAX_ARGS_SIZE)? } else { vec![] };
    let amount = if amount > 0 { read_region(&env.memory(), amount, MAX_IDNA_SIZE)? } else { vec![] };

    deduct_balance_if_needed(env, &amount)?;

    let promise_res = env.promise_then(promise_idx as usize, method, args, amount, gas_limit as u64);
    process_gas_info(env, promise_res.1)?;
    promise_res.0?;
    process_gas_info(env, gas_limit as u64)
}

pub fn create_transfer_promise<B: Backend>(env: &Env<B>, addr: u32, amount: u32) -> VmResult<()> {
    let to = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;
    let amount = read_region(&env.memory(), amount, MAX_IDNA_SIZE)?;
    set_left_gas_to_backend(env)?;
    let (res, gas) = env.backend.deduct_balance(amount.to_vec());
    process_gas_info(env, gas)?;
    res?;
    let promise_res = env.create_transfer_promise(to, amount);
    process_gas_info(env, promise_res.1)?;
    promise_res.0?;
    Ok(())
}

pub fn own_addr<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;
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
    let args = if args > 0 { read_region(&env.memory(), args, MAX_ARGS_SIZE)? } else { vec![] };
    let nonce = if nonce > 0 { read_region(&env.memory(), nonce, MAX_STRING_SIZE)? } else { vec![] };

    set_left_gas_to_backend(env)?;
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
    let args = if args > 0 { read_region(&env.memory(), args, MAX_ARGS_SIZE)? } else { vec![] };
    let nonce = if nonce > 0 { read_region(&env.memory(), nonce, MAX_STRING_SIZE)? } else { vec![] };

    set_left_gas_to_backend(env)?;
    let (res, gas) = env.backend.contract_addr_by_hash(&hash, &args, &nonce);
    process_gas_info(env, gas)?;
    let addr = match res {
        Ok(v) => v,
        Err(err) => return Err(err.into())
    };
    write_to_contract(env, &addr)
}


pub fn own_code<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;
    let (res, gas) = env.backend.own_code();
    process_gas_info(env, gas)?;
    let code = match res {
        Ok(v) => v,
        Err(err) => return Err(err.into())
    };
    write_to_contract(env, &code)
}

pub fn code_hash<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;
    let (res, gas) = env.backend.code_hash();
    process_gas_info(env, gas)?;
    let hash = match res {
        Ok(v) => v,
        Err(err) => return Err(err.into())
    };
    write_to_contract(env, &hash)
}

pub fn create_read_contract_data_promise<B: Backend>(env: &Env<B>, addr: u32, key: u32, gas_limit: u32) -> VmResult<u32> {
    let to = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;
    let key = read_region(&env.memory(), key, MAX_STORAGE_KEY_SIZE)?;

    let idx_res = env.create_read_sharded_data_promise(to, ReadShardedDataAction::ReadContractData(ReadContractDataAction {
        key,
        gas_limit: gas_limit as u64,
    }));
    let idx = idx_res.0?;
    process_gas_info(env, gas_limit as u64)?;
    process_gas_info(env, idx_res.1)?;
    Ok(idx)
}

pub fn create_get_identity_promise<B: Backend>(env: &Env<B>, addr: u32, gas_limit: u32) -> VmResult<u32> {
    let to = read_region(&env.memory(), addr, MAX_ADDRESS_SIZE)?;

    let idx_res = env.create_read_sharded_data_promise(to.clone(), ReadShardedDataAction::GetIdentity(GetIdentityAction {
        addr: to,
        gas_limit: gas_limit as u64,
    }));
    let idx = idx_res.0?;
    process_gas_info(env, gas_limit as u64)?;
    process_gas_info(env, idx_res.1)?;
    Ok(idx)
}

pub fn bytes_to_hex<B: Backend>(env: &Env<B>, ptr: u32) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;
    let data = read_region(&env.memory(), ptr, MAX_ARGS_SIZE)?;
    let str = hex::encode(&data);
    process_gas_info(env, (data.len() as u64) + BASE_BYTES_TO_HEX_COST)?;
    write_to_contract(&env, str.as_bytes())
}

pub fn block_header<B: Backend>(env: &Env<B>, height: u64) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;
    let data = env.backend.block_header(height);
    process_gas_info(env, data.1)?;
    let v = data.0?;
    match v {
        Some(header) => write_to_contract(env, &header),
        None => Ok(0)
    }
}

pub fn keccak256<B: Backend>(env: &Env<B>, ptr: u32) -> VmResult<u32> {
    let data = read_region(&env.memory(), ptr, MAX_ARGS_SIZE)?;
    set_left_gas_to_backend(env)?;
    let hash = env.backend.keccak256(&data);
    process_gas_info(env, hash.1)?;
    let hash_value = hash.0?;
    write_to_contract(env, &hash_value)
}

pub fn global_state<B: Backend>(env: &Env<B>) -> VmResult<u32> {
    set_left_gas_to_backend(env)?;
    let global = env.backend.global_state();
    process_gas_info(env, global.1)?;
    let data = global.0?;
    write_to_contract(env, &data)
}


pub fn gas_limit<B: Backend>(env: &Env<B>) -> VmResult<u64> {
    Ok(env.gas_limit())
}

pub fn gas_left<B: Backend>(env: &Env<B>) -> VmResult<u64> {
    Ok(env.get_gas_left())
}

pub fn burn<B: Backend>(env: &Env<B>, amount: u32) -> VmResult<()> {
    let amount = read_region(&env.memory(), amount, MAX_IDNA_SIZE)?;
    set_left_gas_to_backend(env)?;
    let (res, gas) = env.backend.burn(amount.to_vec());
    process_gas_info(env, gas)?;
    res?;
    Ok(())
}

fn set_left_gas_to_backend<B: Backend>(env: &Env<B>) -> VmResult<()> {
    let gas_left = env.get_gas_left();
    env.backend.set_remaining_gas(gas_left).0?;
    Ok(())
}





