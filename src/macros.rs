#![macro_escape]

#[macro_export]
macro_rules! unwrap_or_return {
    ( $e:expr, $gas:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => return (Err(err), $gas),
        }
    }
}

#[macro_export]
macro_rules! check_go_result {
    ($res:expr, $gas:expr, $msg:expr) => {
        if $res != 0 {
            match $res {
                3 => return (Err(BackendError::out_of_gas()), $gas),
                _ => return (Err(BackendError::new($msg)), $gas)
            }
        }
    }
}

#[macro_export]
macro_rules! unwrap_or_action_res {
    ($e:expr, $input_action:expr, $gas_used:expr, $gas_limit:expr, $contract:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => return ActionResult {
            error: err.to_string(),
            success: false,
            gas_used: $gas_used,
            remaining_gas: $gas_limit.saturating_sub($gas_used),
            input_action: $input_action,
            sub_action_results: vec![],
            output_data: vec![],
            contract : $contract,
        },
        }
    }
}