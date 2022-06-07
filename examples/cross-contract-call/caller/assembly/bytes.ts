import { debug } from "./debug";

export class Bytes extends Uint8Array {
  static fromBytes(data: Uint8Array): Bytes {
    //@ts-ignore
    return changetype<Bytes>(data);
  }

  toHex(): string {
    return this.reduce(
      (output, elem) => output + ("0" + (elem & 0xff).toString(16)),
      ""
    );
  }

  prepend(_elementPrefix: Bytes): Bytes {
    let bs = new Uint8Array(_elementPrefix.length + this.length);
    memory.copy(bs.dataStart, _elementPrefix.dataStart, _elementPrefix.length);
    memory.copy(
      bs.dataStart + _elementPrefix.length,
      this.dataStart,
      this.length
    );
    return Bytes.fromBytes(bs);
  }

  toBytes(): Bytes {
    return this;
  }

  toU64(): u64 {
    let d = new DataView(this.buffer);
    return d.getUint64(0, true);
  }

  static fromU64(n: u64): Bytes {    
    let bs = new Uint8Array(8);
    let d = new DataView(bs.buffer);
    d.setUint64(0, n, true);
    return changetype<Bytes>(bs);
  }
}
