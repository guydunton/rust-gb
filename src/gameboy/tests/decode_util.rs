use crate::gameboy::opcodes::Decoder;
use crate::gameboy::OpCode;

pub fn decode(memory: &[u8]) -> OpCode {
    Decoder::decode_instruction(0x00, memory).unwrap()
}
