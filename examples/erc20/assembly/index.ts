import { Address } from "./address";
import {Bytes} from "./bytes"
import { debug } from "./debug";
import { env } from "./env";
import { PersistentMap, StringKey } from "./persistentMap";
import { Region } from "./region";
import {util} from "./utils";

const balances = new PersistentMap<Address>("b:");
const approves = new PersistentMap<StringKey>("a:");

const TOTAL_SUPPLY: u64 = 7777777;


function assert(value : bool, msg : string) : void {
  if (!value) {
    let r = new Region(util.stringToBytes(msg))
    env.panic(changetype<usize>(r))
  }
}


function ptrToBytes(ptr: i32): Bytes {
  let region = changetype<Region>(ptr);
  return Bytes.fromBytes(region.read())
}

export function deploy(): void {
  let addr = Address.fromBytes(ptrToBytes(env.caller()));
  balances.set(addr, TOTAL_SUPPLY);
}

export function allocate(size: usize): i32 {
  let data = new Uint8Array(size);
  let r = new Region(data);
  let result = changetype<usize>(r);
  debug(`allocate:${size} , ptr=${result}`);
  return result;
}

export function transfer(to: i32, tokens: i32): void {
  let caller = Address.fromBytes(ptrToBytes(env.caller()));
  let dest = Address.fromBytes(ptrToBytes(to));
  let amount = ptrToBytes(tokens).toU64();

  debug(
    "transfer from: " +
      caller.toHex() +
      " to: " +
      dest.toHex() +
      " tokens: " +
      amount.toString()
  );
  const fromAmount = getBalance(caller);
  assert(fromAmount >= amount, "not enough tokens on account");
  let destBalance = getBalance(dest);  
  assert(destBalance <= destBalance + amount, "overflow at the receiver side");
  balances.set(caller, fromAmount - amount);
  balances.set(dest, destBalance + amount);
}

export function approve(spender: i32, tokens: i32): void {
  let caller = Address.fromBytes(ptrToBytes(env.caller()));
  let spenderAddr = Address.fromBytes(ptrToBytes(spender));
  let amount = ptrToBytes(tokens).toU64();
  debug("approve: " + caller.toHex() + " tokens: " + amount.toString());
  approves.set(StringKey.fromString(caller.toHex() + ":" + spenderAddr.toHex()), amount);
}

function allowance(tokenOwner: Address, spender: Address): u64 {
  const key = tokenOwner.toHex() + ":" + spender.toHex();
  return approves.get(StringKey.fromString(key), 0);
}

export function transferFrom(from: i32, to: i32, tokens: i32): void {
  let caller = Address.fromBytes(ptrToBytes(env.caller()));
  let fromAddr = Address.fromBytes(ptrToBytes(from));
  let dest = Address.fromBytes(ptrToBytes(to));
  let amount = ptrToBytes(tokens).toU64();

  const fromAmount = getBalance(fromAddr);
  assert(fromAmount >= amount, "not enough tokens on account");
  const approvedAmount = allowance(fromAddr, caller);
  assert(amount <= approvedAmount, "not enough tokens approved to transfer");

  let destBalance = getBalance(dest);

  assert(destBalance <= destBalance + amount, "overflow at the receiver side");
  balances.set(fromAddr, fromAmount - amount);
  balances.set(dest, destBalance + amount);
}

function getBalance(owner: Address): u64 {
  return balances.get(owner, 0);
}
