import {Address} from "./address";
import {Bytes} from "./bytes"
import {debug} from "./debug";
import {env} from "./env";
import {Region} from "./region";
import {util} from "./utils";
import {KeyValue} from "./keyValue";

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

export function inc(x: i32): i32 {
    let xValue = ptrToBytes(x).toU64()
    debug(`inc xValue=${xValue}`)
    return bytesToPtr(Bytes.fromU64(xValue + 1))
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
    memory.copy(result.dataStart + 8, data.dataStart, data.length)
    return result
}
