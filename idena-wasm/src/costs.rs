use wasmer::wasmparser::Operator;

pub fn cost_function(operator: &Operator) -> u64 {
    match operator {
        Operator::LocalGet { .. } | Operator::I32Const { .. } => 1,
        Operator::I32Add { .. } => 1,
        _ => 1,
    }
}