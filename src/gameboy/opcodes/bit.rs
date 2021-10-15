use crate::gameboy::cpu::CPU;

use super::super::flags_register::{write_flag, Flags};
use super::argument::Argument;
use super::OpCode;

impl OpCode {
    pub fn run_bit(&self, cpu: &mut CPU, _memory: &mut Vec<u8>) -> u32 {
        let mut cycles = 0;
        assert_eq!(self.args.len(), 2);

        match (self.args[0], self.args[1]) {
            (Argument::Bit(bit), Argument::Register8Constant(register)) => {
                let register = cpu.read_8_bits(register);

                let result = (((0x1 << bit) ^ register) >> bit) == 1;
                write_flag(cpu, Flags::Z, result);
                write_flag(cpu, Flags::N, false);
                write_flag(cpu, Flags::H, true);
            }
            _ => panic!("Invalid arguments"),
        }

        cycles += 12;
        cycles
    }
}
