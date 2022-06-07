import {env} from './env'
import {Region} from './region'
import {util} from './utils'

export function debug<T>(msg: T): void {
  let msg_encoded: Uint8Array
  // @ts-ignore
  if (isString<T>() || isDefined(msg.toString)) {
    // @ts-ignore
    let message = msg.toString()
    msg_encoded = util.stringToBytes('WASM-LOG: '.concat(message))
  } else {
    msg_encoded = util.stringToBytes('WASM-LOG: cannot convert msg to string')
  }
  let r = new Region(msg_encoded)

  env.print(changetype<usize>(r))
}
