use super::super::RegisterLabel8;
use super::ReadWriteRegister;
use super::{Argument, OpCode};

impl OpCode {
    pub fn run_xor<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _memory: &mut Vec<u8>,
    ) -> u32 {
        let mut cycles = 0;

        match self.args[0] {
            Argument::Register8Constant(register) => {
                let new_val = cpu.read_8_bits(RegisterLabel8::A) ^ cpu.read_8_bits(register);
                cpu.write_8_bits(RegisterLabel8::A, new_val);
                cpu.write_8_bits(RegisterLabel8::F, 0);
            }
            _ => panic!("Argument not supported: {:?}", self.args[0]),
        }

        cycles += 4;
        cycles
    }
}
