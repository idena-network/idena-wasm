import {Bytes} from "asi/assembly/bytes";
import {debug} from "asi/assembly/debug";
import {util} from "asi/assembly/utils";
export {allocate} from "asi/assembly/allocate";

export function deploy(): void {
}

export function inc(x: i32): i32 {
    let xValue = util.ptrToBytes(x).toU64()
    debug(`inc xValue=${xValue}`)
    return util.bytesToPtr(Bytes.fromU64(xValue + 1))
}