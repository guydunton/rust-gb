use crate::gameboy::cpu::CPU;

use super::super::{Argument, RegisterLabel16};

pub fn run_push(args: &[Argument], cpu: &mut CPU, memory: &mut Vec<u8>) -> u32 {
    let mut cycles = 0;
    if let Argument::Register16Constant(reg) = args[0] {
        let value = cpu.read_16_bits(reg);
        let bytes = value.to_be_bytes();

        let sp = cpu.read_16_bits(RegisterLabel16::StackPointer);
        memory[(sp - 1) as usize] = bytes[0];
        memory[(sp - 2) as usize] = bytes[1];

        cpu.write_16_bits(RegisterLabel16::StackPointer, sp - 2);

        cycles += 16;
    }
    cycles
}
