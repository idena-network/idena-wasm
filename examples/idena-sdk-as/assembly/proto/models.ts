import { Writer, Reader } from "as-proto";

export namespace models {
  export class ProtoStateIdentity {
    static encode(message: ProtoStateIdentity, writer: Writer): void {
      const stake = message.stake;
      if (stake !== null) {
        writer.uint32(10);
        writer.bytes(stake);
      }

      writer.uint32(24);
      writer.uint32(message.birthday);

      writer.uint32(32);
      writer.uint32(message.state);
    }

    static decode(reader: Reader, length: i32): ProtoStateIdentity {
      const end: usize = length < 0 ? reader.end : reader.ptr + length;
      const message = new ProtoStateIdentity();

      while (reader.ptr < end) {
        const tag = reader.uint32();
        switch (tag >>> 3) {
          case 1:
            message.stake = reader.bytes();
            break;

          case 3:
            message.birthday = reader.uint32();
            break;

          case 4:
            message.state = reader.uint32();
            break;

          default:
            reader.skipType(tag & 7);
            break;
        }
      }

      return message;
    }

    stake: Uint8Array | null;
    birthday: u32;
    state: u32;

    constructor(
      stake: Uint8Array | null = null,
      birthday: u32 = 0,
      state: u32 = 0
    ) {
      this.stake = stake;
      this.birthday = birthday;
      this.state = state;
    }
  }

  export class ProtoTransactionIndex {
    static encode(message: ProtoTransactionIndex, writer: Writer): void {
      const blockHash = message.blockHash;
      if (blockHash !== null) {
        writer.uint32(10);
        writer.bytes(blockHash);
      }

      writer.uint32(16);
      writer.uint32(message.idx);
    }

    static decode(reader: Reader, length: i32): ProtoTransactionIndex {
      const end: usize = length < 0 ? reader.end : reader.ptr + length;
      const message = new ProtoTransactionIndex();

      while (reader.ptr < end) {
        const tag = reader.uint32();
        switch (tag >>> 3) {
          case 1:
            message.blockHash = reader.bytes();
            break;

          case 2:
            message.idx = reader.uint32();
            break;

          default:
            reader.skipType(tag & 7);
            break;
        }
      }

      return message;
    }

    blockHash: Uint8Array | null;
    idx: u32;

    constructor(blockHash: Uint8Array | null = null, idx: u32 = 0) {
      this.blockHash = blockHash;
      this.idx = idx;
    }
  }
}
