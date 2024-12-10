use super::super::argument::Argument;

use crate::gameboy::cpu::CPU;
use crate::gameboy::opcodes::JumpCondition;
use crate::gameboy::{read_flag, Flags, RegisterLabel16};

pub fn run_call(args: &[Argument], cpu: &mut CPU, memory: &mut [u8]) -> u32 {
    let mut cycles = 12;

    let should_jump = match args[0] {
        Argument::JumpCondition(JumpCondition::Carry) => read_flag(cpu, Flags::C),
        Argument::JumpCondition(JumpCondition::NotCarry) => !read_flag(cpu, Flags::C),
        Argument::JumpCondition(JumpCondition::Zero) => read_flag(cpu, Flags::Z),
        Argument::JumpCondition(JumpCondition::NotZero) => !read_flag(cpu, Flags::Z),
        _ => true,
    };

    let address = match (args[0], args[1]) {
        (Argument::Label(address), _) => address,
        (Argument::JumpCondition(_), Argument::Label(address)) => address,
        _ => panic!("Unsupported call arguments {:?}, {:?}", args[0], args[1]),
    };

    if should_jump {
        // Store the contents of the program counter on the stack
        let pc = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        let return_address = pc.to_be_bytes();

        let stack = cpu.read_16_bits(RegisterLabel16::StackPointer);
        memory[(stack - 1) as usize] = return_address[0];
        memory[(stack - 2) as usize] = return_address[1];

        // Update the stack
        cpu.write_16_bits(RegisterLabel16::StackPointer, stack - 2);

        // Move the program counter to the value of the argument
        cpu.write_16_bits(RegisterLabel16::ProgramCounter, address);

        cycles += 12;
    }

    cycles
}
