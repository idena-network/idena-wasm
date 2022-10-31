use protobuf::Message;

use crate::errors::VmError;
use crate::memory::VmResult;
use crate::proto;

const ARGS_PROTOBUF_FORMAT: u8 = 0x1;
const ARGS_PLAIN_FORMAT: u8 = 0x0;


pub fn convert_args(args: &[u8]) -> VmResult<protobuf::RepeatedField<proto::models::ProtoArgs_Argument>> {
    let mut result: protobuf::RepeatedField<proto::models::ProtoArgs_Argument>;

    match args[0] {
        ARGS_PROTOBUF_FORMAT => {
            result = proto::models::ProtoArgs::parse_from_bytes(&args[1..])
                .or(Err(VmError::custom("failed to parse arguments")))?.args;
        }
        ARGS_PLAIN_FORMAT => {
            let mut arg = proto::models::ProtoArgs_Argument::new();
            arg.set_value(args[1..].to_vec());
            result = protobuf::RepeatedField::<proto::models::ProtoArgs_Argument>::new();
            result.push(arg);
        }
        _ => return Err(VmError::custom("unknown format of args"))
    }
    Ok(result)
}
