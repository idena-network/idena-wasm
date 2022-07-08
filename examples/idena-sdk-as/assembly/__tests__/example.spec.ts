import { Protobuf } from 'as-proto';
import { models } from "../proto/models";
import { util } from "../utils";
import { encode, decode } from 'as-hex'

class Vec3 {
  constructor(public x: f64 = 0, public y: f64 = 0, public z: f64 = 0) {}
}
describe("example", () => {
  // it("should be 42", () => {
  //   expect<i32>(19 + 23).toBe(42, "19 + 23 is 42");
  // });

  // it("should be the same reference", () => {
  //   let ref = new Vec3();
  //   expect<Vec3>(ref).toBe(ref, "Reference Equality");
  // });

  // it("should perform a memory comparison", () => {
  //   let a = new Vec3(1, 2, 3);
  //   let b = new Vec3(1, 2, 3);

  //   expect<Vec3>(a).toStrictEqual(
  //     b,
  //     "a and b have the same values, (discluding child references)",
  //   );
  // });

  // it("should compare strings", () => {
  //   expect<string>("a=" + "200").toBe("a=200", "both strings are equal");
  // });

  // it("should compare values", () => {
  //   expect<i32>(10).toBeLessThan(200);
  //   expect<i32>(1000).toBeGreaterThan(200);
  //   expect<i32>(1000).toBeGreaterThanOrEqual(1000);
  //   expect<i32>(1000).toBeLessThanOrEqual(1000);
  // });

  // it("can log some values to the console", () => {
  //   log<string>("Hello world!"); // strings!
  //   log<f64>(3.1415); // floats!
  //   log<u8>(244); // integers!
  //   log<u64>(0xffffffff); // long values!
  //   log<ArrayBuffer>(new ArrayBuffer(50)); // bytes!
  // });

  it("test test test", () => {


    const a = util.decodeFromHex("0a09482446b7322c992ffc1002182d2008282d30593a4104c15f9a61d46fb02ccc2cc463c24d5afbc1ad7c4d5846ba25fd02fe4ac33eb97ffeb9b001c9ca4319354089cd04fdff9a1ec36cd959cdd357d3e4521d3f268a3b400350025a0c8843270812eece07bfb8cbe862380a206c5b685b6e02386a48ec31e6fad95be8767b27ec4d94a2832016601bf81ec6da12144e9878abfded6d35e8d7de9618e5b81d77b72f6e92010ac6c6a5c6c6c6a5c684c6a00103b00102")
    
    log(a)

    const identity = Protobuf.decode<models.ProtoStateIdentity>(a, models.ProtoStateIdentity.decode);

    log<models.ProtoStateIdentity>(identity); // strings!
  });
});
