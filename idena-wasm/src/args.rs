use protobuf::Message;

use crate::errors::VmError;
use crate::memory::VmResult;
use crate::proto;
use crate::proto::models::ProtoCallContractArgs;

const ARGS_PROTOBUF_FORMAT: u8 = 0x1;
const ARGS_PLAIN_FORMAT: u8 = 0x0;


pub fn convert_args(args: &[u8]) -> VmResult<protobuf::RepeatedField<Vec<u8>>> {
    let result: protobuf::RepeatedField<Vec<u8>>;

    match args[0] {
        ARGS_PROTOBUF_FORMAT => {
            result = proto::models::ProtoCallContractArgs::parse_from_bytes(&args[1..])
                .or(Err(VmError::custom("failed to parse arguments")))?.args;
        }
        ARGS_PLAIN_FORMAT => { result = protobuf::RepeatedField::from_vec(vec![args[1..].to_vec()]); }
        _ => return Err(VmError::custom("unknown format of args"))
    }
    Ok(result)
}
