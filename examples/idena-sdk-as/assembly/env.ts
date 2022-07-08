export namespace env {
    //@ts-ignore
    @external("env", "debug")
    // Prints value to console, not available in production
    export declare function print(key: i32): void

    @external("env", "panic")
    // Interrupts execution with error message
    export declare function panic(msg: i32): void

    //@ts-ignore
    @external("env", "set_storage")
    // Sets key-value pair to contract store
    export declare function setStorage(key: i32, value: i32): void

    //@ts-ignore
    @external("env", "get_storage")
    // Reads value by key from contract store
    export declare function getStorage(key: i32): i32

    //@ts-ignore
    @external("env", "remove_storage")
    // Removes key-value pair from contract store
    export declare function removeStorage(key: i32): void

    //@ts-ignore
    @external("env", "block_timestamp")
    // Reads head block timestamp
    export declare function blockTimestamp(): i64

    //@ts-ignore
    @external("env", "block_seed")
    // Reads head block seed
    export declare function blockSeed(): i64

    //@ts-ignore
    @external("env", "block_number")
    // Reads head block number
    export declare function blockNumber(): u64

    //@ts-ignore
    @external("env", "min_fee_per_gas")
    // Reads minimal fee per gas
    export declare function minFeePerGas(): i32

    //@ts-ignore
    @external("env", "network_size")
    // Reads current network size
    export declare function networkSize(): u64

    //@ts-ignore
    @external("env", "identity")
    /**
     * Reads full protobuf model of identity
     * @returns pointer to region with binary data, 0 if identity doesn't exist
     */
    export declare function identity(addr: usize): i32

    //@ts-ignore
    @external("env", "identity_state")
    // Reads identity status by address
    export declare function identityState(addr: usize): i32

    //@ts-ignore
    @external("env", "caller")
    // Reads predecessor of contract
    export declare function caller(): i32

    //@ts-ignore
    @external("env", "originalCaller")
    // Reads signer of original transaction
    export declare function originalCaller(): i32

    //@ts-ignore
    @external("env", "create_call_function_promise")
    export declare function createCallFunctionPromise(contract: usize, method: usize, args: usize, deposit: usize, gasLimit: usize): i32

    //@ts-ignore
    @external("env", "create_deploy_contract_promise")
    export declare function createDeployContractPromise(code: usize, args: usize, nonce: usize, deposit: usize, gasLimit: usize): i32

    //@ts-ignore
    @external("env", "create_transfer_promise")
    export declare function createTransferPromise(to: usize, amount: usize) : void

    //@ts-ignore
    @external("env", "promise_then")
    // Creates callback that will be executed after promise has finished
    export declare function then(promiseIdx : i32, method: usize, args: usize, amount: usize, gasLimit: usize) : void

    //@ts-ignore
    @external("env", "promise_result")
    /**
    Reads value of promise.
    @param result contains binary data of value.
    @returns status of promise result. 0 - failed, 1 - empty value, 2 - some value
    */
    export declare function promiseResult(result : usize): i32

    //@ts-ignore
    @external("env", "contract")
    // Reads address of current contract
    export declare function contract(): i32

    //@ts-ignore
    @external("env", "balance")
    // Reads balance of addr
    export declare function balance(addr: usize): i32
}
