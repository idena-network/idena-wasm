import { Bytes } from "./bytes";
import { env } from "./env";
import { Region } from "./region";
import { util } from "./utils";

export interface Encodable {
  toBytes(): Bytes;
}

export interface Decodable {
  fromBytes(data : Bytes): void;
}



export class StringKey implements Encodable {
  private value : string
  constructor(str : string) {
    this.value = str
  }


  static fromString(str: string): StringKey {
    return new StringKey(str)
  }

  toBytes(): Bytes {
    return Bytes.fromBytes(util.stringToBytes(this.value));
  }
}

export class PersistentMap<K extends Encodable> {
  private _elementPrefix: Bytes;

  constructor(prefix: string) {
    this._elementPrefix = Bytes.fromBytes(util.stringToBytes(prefix));
  }

  private _key(key: K): string {
    // @ts-ignore
    return this._elementPrefix + key.toString();
  }

  private _encodeKey(key: K): usize {
    let keyBytes = key.toBytes();
    keyBytes = keyBytes.prepend(this._elementPrefix);
    return changetype<usize>(new Region(keyBytes));
  }

  delete(key: K): void {
    env.removeStorage(this._encodeKey(key));
  }

  get(key: K, defaultValue: u64): u64 {
    let valuePtr = env.getStorage(this._encodeKey(key));
    if (valuePtr == 0) {
      return defaultValue;
    }
    let value = changetype<Region>(valuePtr);
    let bytes = Bytes.fromBytes(value.read());
    return bytes.toU64();
  }

  set(key: K, value: u64): void {
    // @ts-ignore
    let valueRegion = changetype<usize>(new Region(Bytes.fromU64(value)));
    env.setStorage(this._encodeKey(key), valueRegion);
  }
}
