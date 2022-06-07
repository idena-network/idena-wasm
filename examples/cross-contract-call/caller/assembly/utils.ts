export namespace util {
  /**
   * Convert a given string into a Uint8Array encoded as UTF-8.
   * @param s data to encode
   */
  export function stringToBytes(s: string): Uint8Array {
    let len = String.UTF8.byteLength(s, true) - 1
    let bytes = new Uint8Array(len)
    memory.copy(bytes.dataStart, toUTF8(s), len)
    return bytes
  }

  /**
   * Decode an UTF-8 encoded Uint8Array into a string.
   * @param bytes array to decode
   */
  export function bytesToString(bytes: Uint8Array | null): string | null {
    if (bytes == null) {
      return null
    }
    return String.UTF8.decode(uint8ArrayToBuffer(bytes), true)
  }

  /**
   * Calculates the byte length of the specified UTF-8 string, which can optionally be null terminated.
   * @param str data
   * @param nullTerminated
   */
  export function UTF8Length(str: string, nullTerminated = false): usize {
    return String.UTF8.byteLength(str, nullTerminated)
  }

  /**
   * Parses the given bytes array to return a value of the given generic type.
   * Supported types: bool, integer, string and data objects defined in model.ts.
   *
   * @param bytes Bytes to parse. Bytes must be not null.
   * @returns A parsed value of type T.
   */
  // export function parseFromBytes<T>(bytes: Uint8Array): T {
  //   return decode<T>(bytes);
  // }

  /**
   * Parses the given string to return a value of the given generic type.
   * Supported types: bool, integer, string and data objects defined in model.ts.
   *
   * @param s String to parse. Must be not null.
   * @returns A parsed value of type T.
   */
  export function parseFromString<T>(s: string): T {
    if (isString<T>()) {
      // @ts-ignore
      return s
    } else if (isInteger<T>()) {
      if (isBoolean<T>()) {
        // @ts-ignore
        return <T>(s == 'true')
      } else if (isSigned<T>()) {
        // @ts-ignore
        return <T>I64.parseInt(s)
      } else {
        // @ts-ignore
        return <T>U64.parseInt(s)
      }
    } else {
      // @ts-ignore v will have decode method
      return decode<T>(stringToBytes(s))
    }
  }

  export function decodeFromHex(str: string): Uint8Array {
    let s = stripHexPrefix(str)
    let array = new Uint8Array(s.length >>> 1)

    for (let i = 0; i < s.length >>> 1; ++i) {
      array.fill(i32(I64.parseInt('0x' + s.substr(i * 2, 2), 16)), i, i + 1)
    }

    return array
  }

  export function encodeToHex(
    data: Uint8Array,
    withPrefix: bool = false
  ): string {
    let hex = ''

    for (let i = 0; i < data.length; i++) {
      hex += data[i].toString(16)
    }

    return withPrefix ? '0x' + hex : hex
  }

  // Private helpers
  function toUTF8(str: string, nullTerminated: boolean = false): usize {
    return changetype<usize>(String.UTF8.encode(str, nullTerminated))
  }

  function uint8ArrayToBuffer(array: Uint8Array): ArrayBuffer {
    return array.buffer.slice(
      array.byteOffset,
      array.byteLength + array.byteOffset
    )
  }

  function isHexPrefixed(str: string): bool {
    return str.slice(0, 2) === '0x'
  }

  function stripHexPrefix(str: string): string {
    return isHexPrefixed(str) ? str.slice(2) : str
  }
}
