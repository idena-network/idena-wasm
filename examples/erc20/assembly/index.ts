import {Address} from "asi/assembly/address";
import {decodeU64, encodeAddress, encodeString, encodeU64} from "asi/assembly/bytes"
import { debug } from "asi/assembly/debug";
import { env } from "./env";
import { PersistentMap } from "asi/assembly/persistentMap";
import {util} from "asi/assembly/utils";

const balances = new PersistentMap<Address, u64>("b:",  encodeAddress, encodeU64, decodeU64);
const approves = new PersistentMap<string, u64>("a:", encodeString, encodeU64, decodeU64);

const TOTAL_SUPPLY: u64 = 7777777;

export {allocate} from "asi/assembly/allocate";

export function deploy(): void {
  let addr = Address.fromBytes(util.ptrToBytes(env.caller()));
  balances.set(addr, TOTAL_SUPPLY);
}

export function transfer(to: i32, tokens: i32): void {
  let caller = Address.fromBytes(util.ptrToBytes(env.caller()));
  let dest = Address.fromBytes(util.ptrToBytes(to));
  let amount = util.ptrToBytes(tokens).toU64();

  debug(
    "transfer from: " +
      caller.toHex() +
      " to: " +
      dest.toHex() +
      " tokens: " +
      amount.toString()
  );
  const fromAmount = getBalance(caller);
  util.assert(fromAmount >= amount, "not enough tokens on account");
  let destBalance = getBalance(dest);
  util.assert(destBalance <= destBalance + amount, "overflow at the receiver side");
  balances.set(caller,fromAmount - amount);
  balances.set(dest, destBalance + amount);
}

export function approve(spender: i32, tokens: i32): void {
  let caller = Address.fromBytes(util.ptrToBytes(env.caller()));
  let spenderAddr = Address.fromBytes(util.ptrToBytes(spender));
  let amount = util.ptrToBytes(tokens).toU64();
  debug("approve: " + caller.toHex() + " tokens: " + amount.toString());
  approves.set(caller.toHex() + ":" + spenderAddr.toHex(), amount);
}

function allowance(tokenOwner: Address, spender: Address): u64 {
  const key = tokenOwner.toHex() + ":" + spender.toHex();
  return approves.get(key, 0);
}

export function transferFrom(from: i32, to: i32, tokens: i32): void {
  let caller = Address.fromBytes(util.ptrToBytes(env.caller()));
  let fromAddr = Address.fromBytes(util.ptrToBytes(from));
  let dest = Address.fromBytes(util.ptrToBytes(to));
  let amount = util.ptrToBytes(tokens).toU64();

  const fromAmount = getBalance(fromAddr);
  util.assert(fromAmount >= amount, "not enough tokens on account");
  const approvedAmount = allowance(fromAddr, caller);
  util.assert(amount <= approvedAmount, "not enough tokens approved to transfer");

  let destBalance = getBalance(dest);

  util.assert(destBalance <= destBalance + amount, "overflow at the receiver side");
  balances.set(fromAddr, fromAmount - amount);
  balances.set(dest, destBalance + amount);
}

function getBalance(owner: Address): u64 {
  return balances.get(owner, 0);
}
