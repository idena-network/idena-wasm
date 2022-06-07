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

export function deploy(functionAddr: i32): void {
    let addr = Address.fromBytes(ptrToBytes(functionAddr));

    debug(`deploy arg = ${addr}`)
    contractAddr.set(addr)
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

export function _sum(y: i32): void {
    let yValue = ptrToBytes(y).toU64()

    let xPtr = allocate(8)

    let promiseResult = env.promiseResult(xPtr)

    assert(promiseResult == 2, "promise result should be successful")

    let xValue = ptrToBytes(xPtr).toU64()

    debug(`x=${xValue}, y=${yValue}, sum= ${xValue + yValue}`)
}

export function invoke(x: i32, y: i32): void {
    let addr = contractAddr.get(null, a => Address.fromBytes(a));
    assert(addr!=null && addr.length > 0, "contract should be specified")

    let xValue = ptrToBytes(x).toU64()
    let yValue = ptrToBytes(y).toU64()

    debug(`x=${xValue}, y=${yValue}`)

    if (addr != null) {
        debug(`contract addr = ${addr.toHex()}`)

        let methodPtr = strToPtr("inc")
        debug(`methodPtr = ${methodPtr}`)

        let contractPtr = bytesToPtr(addr.toBytes())
        debug(`contractPtr = ${contractPtr}`)

        let argsPrt = bytesToPtr(packPlainArgument(ptrToBytes(x)))
        debug(`argsPrt = ${argsPrt}`)

        let promiseIdx = env.createCallFunctionPromise(contractPtr, methodPtr, argsPrt , 0, 100000)

        debug(`created promise=${promiseIdx}`)

        let sumMethodPtr =  strToPtr("_sum")
        debug(`sum methodPtr = ${sumMethodPtr}`)

        let sumArgPtr= bytesToPtr(packPlainArgument(ptrToBytes(y)))
        debug(`sum arg ptr  = ${sumArgPtr}`)

        env.then(promiseIdx, sumMethodPtr , sumArgPtr, 0, 100000)
    }
}
