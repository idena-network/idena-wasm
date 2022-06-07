export class Region {
  offset: u32
  len: u32
  capacity: u32
  constructor(data: Uint8Array) {
    this.offset = data.dataStart
    this.len = data.length
    this.capacity = data.length
  }
  read(): Uint8Array {
    let data = new Uint8Array(this.len)
    memory.copy(data.dataStart, this.offset, this.len)
    return data
  }
}
