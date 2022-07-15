import {Address} from "asi/assembly/address";
import {KeyValue} from "asi/assembly/keyValue";
import {Region} from "asi/assembly/region";
import {util} from "asi/assembly/utils";
import {env} from "asi/assembly/env";
import {Bytes} from "asi/assembly/bytes";
import {debug} from "asi/assembly/debug";


var contractAddr = new KeyValue<Address>("contract")

function assert(value: bool, msg: string): void {
    if (!value) {
        let r = new Region(util.stringToBytes(msg))
        env.panic(changetype<usize>(r))
    }
}

function ptrToBytes(ptr: i32): Bytes {
    let region = changetype<Region>(ptr);
    return Bytes.fromBytes(region.read())
}

function bytesToPtr(data: Bytes): usize {
    let r = new Region(data)
    return changetype<usize>(r)
}

function strToPtr(data: string): usize {
    let r = new Region(util.stringToBytes(data))
    return changetype<usize>(r)
}

export function deploy(): void {
}

export function allocate(size: usize): i32 {
    let data = new Uint8Array(size);
    let r = new Region(data);
    let result = changetype<usize>(r);
    debug(`allocate:${size} , ptr=${result}`);
    return result;
}

function packPlainArgument(data: Bytes): Bytes {
    var result = new Bytes(data.length + 1);
    result[0] = 0 // plain format
    memory.copy(result.dataStart + 1, data.dataStart, data.length)
    return result
}

export function test(testcasePtr: usize, dataPtr: usize): void {
    var testCase = ptrToBytes(testcasePtr).toU32()
    debug(`run test case = ${testCase}`)
    const deploy_another_contract: u32 = 1;
    switch (testCase) {
        case deploy_another_contract :
            let deposit = new Bytes(10)
            deposit[0] = 1
            let arg = new Bytes(20)
            arg[0] = 4
            arg[1] = 3
            arg[2] = 17

            env.createDeployContractPromise(dataPtr, bytesToPtr(packPlainArgument(arg)), 0, bytesToPtr(deposit), 1000000)
            break
        default:
            env.panic(strToPtr("unknown test case"));
    }
}

