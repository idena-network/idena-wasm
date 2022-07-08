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