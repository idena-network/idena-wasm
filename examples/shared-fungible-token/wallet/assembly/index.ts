import {Address} from "asi/assembly/address";
import {KeyValue} from "asi/assembly/keyValue";
import {Region} from "asi/assembly/region";
import {util} from "asi/assembly/utils";
import {env} from "asi/assembly/env";
import {Bytes} from "asi/assembly/bytes";
import {debug} from "asi/assembly/debug";


var owner = new KeyValue<Address>("o")
var root = new KeyValue<Address>("r")


function assert(value: bool, msg: string): void {
    if (!value) {
        let r = new Region(util.stringToBytes(msg))
        env.panic(changetype<usize>(r))
    }
}


function ptrToBytes(ptr: usize): Bytes {
    let region = changetype<Region>(ptr);
    return Bytes.fromBytes(region.read())
}

function bytesToPtr(data: Bytes): usize {
    let r = new Region(data)
    debug(`bytes to ptr(${r.offset},${r.capacity},${r.len})`);
    return changetype<usize>(r)
}

function strToPtr(data: string): usize {
    let r = new Region(util.stringToBytes(data))
    return changetype<usize>(r)
}

export function deploy(ownerAddr: i32, rootAddr: i32): void {
    let addr = Address.fromBytes(ptrToBytes(ownerAddr));
    owner.set(addr)

    debug(`owner = ${addr.toHex()}`)

    let rootAddress = Address.fromBytes(ptrToBytes(rootAddr))
    debug(`root = ${rootAddress.toHex()}`)
    root.set(rootAddress)
}

export function allocate(size: usize): usize {
    let data = new Uint8Array(size);
    let r = new Region(data);
    let result = changetype<usize>(r);
    debug(`allocate:${size} , ptr=${result} r=(${r.offset},${r.capacity},${r.len})`);
    return result;
}

export function transferTo(recipient: i32, amount: i32): void {

    let tokens = ptrToBytes(amount).toU64()

    assert(balance() >= tokens, "not enough tokens on account");
    assert(tokens > 0, "amount should be positive");

    let ownerAddr = changetype<Address>(owner.get(Address.fromBytes(new Bytes(0)), a => Address.fromBytes(a)))

    assert(ptrToBytes(env.caller()).toHex() == ownerAddr.toHex(), "sender is not an owner")

    let rootAddr = changetype<Address>(root.get(Address.fromBytes(new Bytes(0)), a => Address.fromBytes(a)))

    let recipientAddr = ptrToBytes(recipient)

    let code = ptrToBytes(env.code())
    let codePtr = bytesToPtr(code)

    let walletArgs = bytesToPtr(util.packProtobufArgument([recipientAddr, rootAddr]));

    let idx = env.createDeployContractPromise(codePtr, walletArgs, 0, 0, 450000)
    var callbackArgs = bytesToPtr(util.packProtobufArgument([recipientAddr, Bytes.fromU64(tokens)]));

    env.then(idx, strToPtr("_deploy_wallet_callback"), callbackArgs, 0, 3000000)
}

function balance(): u64 {
    let ptr = env.getStorage(strToPtr("tokens"))
    if (ptr == 0) {
        return 0
    }
    let balance = ptrToBytes(ptr).toU64()
    debug(`current balance=${balance}`)
    return balance
}

function addBalance(tokens: u64): void {
    debug(`adding balance`)
    let b = balance()
    debug(`current balance ${b}`)
    assert(b + tokens > b, "overflow")
    debug(`assert passed`)
    b = b + tokens;
    debug(`new balance ${b}`)
    env.setStorage(strToPtr("tokens"), bytesToPtr(Bytes.fromU64(b)))
    debug(`received tokens`)
}

function subBalance(tokens: u64): void {
    let b = balance()
    b = b - tokens;
    env.setStorage(strToPtr("tokens"), bytesToPtr(Bytes.fromU64(b)))
}


export function receive(amount: i32, sender_owner: i32): void {
    let tokens = ptrToBytes(amount).toU64()
    let caller = ptrToBytes(env.caller())
    let rootAddr = changetype<Address>(root.get(Address.fromBytes(new Bytes(0)), a => Address.fromBytes(a)))
    let packedArgs = util.packProtobufArgument([ptrToBytes(sender_owner), rootAddr])
    let walletArgs = bytesToPtr(packedArgs)
    var requiredCaller = ptrToBytes(env.contractAddressByHash(env.codeHash(), walletArgs, 0))
    debug(`caller=${caller.toHex()} requiredCaller=${requiredCaller.toHex()}`)
    assert(caller.toHex() == requiredCaller.toHex(), "sender is invalid")

    addBalance(tokens)

    debug(`received tokens`)
}

export function _deploy_wallet_callback(recipient: i32, amount: i32): void {

    let ownerAddr = changetype<Address>(owner.get(Address.fromBytes(new Bytes(0)), a => Address.fromBytes(a)))

    let recipientAddr = ptrToBytes(recipient)
    let rootAddr = changetype<Address>(root.get(Address.fromBytes(new Bytes(0)), a => Address.fromBytes(a)))

    let walletArgs = bytesToPtr(util.packProtobufArgument([recipientAddr, rootAddr]))

    let receiveArgs = bytesToPtr(util.packProtobufArgument([ptrToBytes(amount), ownerAddr]))

    let tokens = ptrToBytes(amount).toU64()
    assert(balance() >= tokens, "not enough tokens on account");
    subBalance(tokens)

    let destination = env.contractAddressByHash(env.codeHash(), walletArgs, 0)

    let idx = env.createCallFunctionPromise(destination, strToPtr("receive"), receiveArgs, 0, 1400000)
    env.then(idx, strToPtr("_send_tokens_callback"), bytesToPtr(util.packPlainArgument(ptrToBytes(amount))), 0, 400000)
}

export function _send_tokens_callback(amount: i32): void {
    let result = env.promiseResult(0)
    if (result == 0) {
        addBalance(ptrToBytes(amount).toU64())
    }
}

