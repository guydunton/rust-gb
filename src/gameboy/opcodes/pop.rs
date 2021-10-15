use crate::gameboy::cpu::CPU;

use super::{Argument, OpCode, RegisterLabel16};

impl OpCode {
    pub fn run_pop(&self, cpu: &mut CPU, memory: &mut Vec<u8>) -> u32 {
        if let Argument::Register16Constant(_) = self.args[0] {
            // Read the stack pointer
            let sp = cpu.read_16_bits(RegisterLabel16::StackPointer);

            // Get the value at the stack pointer
            let lower_byte = memory[sp as usize] as u16;
            let higher_byte = memory[sp as usize + 1] as u16;

            let result = (higher_byte << 8) + lower_byte;

            // Write the result into the BC register
            cpu.write_16_bits(RegisterLabel16::BC, result);

            // Safely add 2 and write away
            cpu.write_16_bits(RegisterLabel16::StackPointer, sp + 2);
        } else {
            panic!("Unknown argument found in pop opcode");
        }
        12
    }
}
