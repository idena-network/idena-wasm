import {Bytes} from "./bytes";
import {util} from "./utils";
import {Region} from "./region";
import {env} from "./env";


export interface Convertable {
    toBytes(): Bytes;
}

export class KeyValue<V extends Convertable> {
    private key: Bytes;

    constructor(key: string) {
        this.key = Bytes.fromBytes(util.stringToBytes(key))
    }

    private _encodeKey(): usize {
        return changetype<usize>(new Region(this.key));
    }

    set(value: V): void {
        env.setStorage(this._encodeKey(), changetype<usize>(new Region(value.toBytes())));
    }

    get(defaultValue: V | null, create: (a: Bytes) => V): V | null {
        let ptr = env.getStorage(this._encodeKey());
        if (ptr == 0) {
            return defaultValue
        }
        let bytes = Bytes.fromBytes(changetype<Region>(ptr).read())
        let a = create(bytes)
        return a
    }
}