export namespace env {
    //@ts-ignore
    @external("env", "debug")
    export declare function print(key: i32): void

    //@ts-ignore
    @external("env", "set_storage")
    export declare function setStorage(key: i32, value: i32): void

    //@ts-ignore
    @external("env", "get_storage")
    export declare function getStorage(key: i32): i32

    //@ts-ignore
    @external("env", "remove_storage")
    export declare function removeStorage(key: i32): void

    //@ts-ignore
    @external("env", "block_timestamp")
    export declare function blockTimestamp(): i64

    //@ts-ignore
    @external("env", "block_number")
    export declare function blockNumber(): u64

    //@ts-ignore
    @external("env", "network_size")
    export declare function networkSize(): u64

    //@ts-ignore
    @external("env", "identity_state")
    export declare function identityState(addr: usize): i32

    //@ts-ignore
    @external("env", "send")
    export declare function send(to: usize, amount: usize): void

    //@ts-ignore
    @external("env", "caller")
    export declare function caller() : i32

    //@ts-ignore
    @external("env", "originCaller")
    export declare function originCaller() : i32
}
