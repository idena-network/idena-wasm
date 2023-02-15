#![allow(unused_variables)]

use wasmer::wasmparser::Operator;

pub const BASE_PROMISE_COST: u64 = 100000;

pub const BASE_DEPLOY_COST: u64 = 3000000;
pub const BASE_CALL_COST: u64 = 100000;

pub const BASE_BYTES_TO_HEX_COST :u64 = 10000;


pub fn cost_function(operator: &Operator) -> u64 {
    1
    /*match operator {
        Operator::LocalGet { .. } | Operator::I32Const { .. } => 1,
        Operator::I32Add { .. } => 1,
        _ => 1,
    }*/
}