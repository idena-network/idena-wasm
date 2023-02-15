#![allow(dead_code)]


static CONTRACT_ERC20: &[u8] = include_bytes!("../testdata/erc20.wasm");



#[test]
fn test_deploy_erc20() {
    let mock_backed = crate::backend::MockBackend {};
    let mut gas_used = 0;
    assert!(crate::runner::VmRunner::deploy_with_env(mock_backed, CONTRACT_ERC20.to_vec(), protobuf::RepeatedField::new(), 100000, &mut gas_used).is_ok());
}