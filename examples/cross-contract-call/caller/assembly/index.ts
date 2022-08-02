import {Address} from "asi/assembly/address";
import {KeyValue} from "asi/assembly/keyValue";
import {util} from "asi/assembly/utils";
import {env} from "asi/assembly/env";
import {debug} from "asi/assembly/debug";
import {decodeAddress, encodeAddress, encodeString} from "asi/assembly/bytes";
import {allocate} from "asi/assembly/allocate";

var contractAddr = new KeyValue<string, Address>("contract", encodeString, encodeAddress, decodeAddress)

export {allocate} from "asi/assembly/allocate";

export function deploy(functionAddr: i32): void {
    let addr = Address.fromBytes(util.ptrToBytes(functionAddr));

    debug(`deploy arg = ${addr}`)
    contractAddr.set(addr)
}

export function _sum(y: i32): void {
    let yValue = util.ptrToBytes(y).toU64()

    let xPtr = allocate(8)

    let promiseResult = env.promiseResult(xPtr)

    util.assert(promiseResult == 2, "promise result should be successful")

    let xValue = util.ptrToBytes(xPtr).toU64()

    debug(`x=${xValue}, y=${yValue}, sum=${xValue + yValue}`)
}

export function invoke(x: i32, y: i32): void {
    let addr = contractAddr.get(new Address(0));
    util.assert(addr != null && addr.length > 0, "contract should be specified")

    let xValue = util.ptrToBytes(x).toU64()
    let yValue = util.ptrToBytes(y).toU64()

    debug(`x=${xValue}, y=${yValue}`)

    if (addr != null) {
        debug(`contract addr = ${addr.toHex()}`)

        let methodPtr = util.strToPtr("inc")
        debug(`methodPtr = ${methodPtr}`)

        let contractPtr = util.bytesToPtr(addr.toBytes())
        debug(`contractPtr = ${contractPtr}`)

        let argsPrt = util.bytesToPtr(util.packPlainArgument(util.ptrToBytes(x)))
        debug(`argsPrt = ${argsPrt}`)

        let promiseIdx = env.createCallFunctionPromise(contractPtr, methodPtr, argsPrt, 0, 100000)

        debug(`created promise=${promiseIdx}`)

        let sumMethodPtr = util.strToPtr("_sum")

        debug(`sum methodPtr = ${sumMethodPtr}`)

        // let sumArgPtr = bytesToPtr(util.packPlainArgument(ptrToBytes(y)))
        let sumArgPtr = util.bytesToPtr(util.packProtobufArgument([util.ptrToBytes(y)]))

        debug(`sum arg ptr  = ${sumArgPtr}`)

        env.then(promiseIdx, sumMethodPtr, sumArgPtr, 0, 100000)
    }
}
