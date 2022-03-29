import { Bytes } from "./bytes";

export class Address extends Bytes {
  static fromBytes(data: Uint8Array): Address {
    return changetype<Address>(data);
  }

  toHex(): string {
    return this.reduce(
      (output, elem) => output + ("0" + (elem & 0xff).toString(16)),
      ""
    );
  }
  toString(): string {
    return this.toHex();
  }
}
