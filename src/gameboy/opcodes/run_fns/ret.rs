use crate::gameboy::cpu::CPU;

use crate::gameboy::opcodes::{Argument, JumpCondition};
use crate::gameboy::{read_flag, Flags, RegisterLabel16};

pub fn run_ret(args: &[Argument], cpu: &mut CPU, memory: &mut [u8]) -> u32 {
    // If there is a condition then check it

    let mut extra_cycles = 0;

    let should_return = match args[0] {
        Argument::JumpCondition(condition) => {
            extra_cycles += 4;
            match condition {
                JumpCondition::NotZero => !read_flag(cpu, Flags::Z),
                JumpCondition::Zero => read_flag(cpu, Flags::Z),
                JumpCondition::Carry => read_flag(cpu, Flags::C),
            }
        }
        _ => true,
    };

    if should_return {
        perform_return(cpu, memory);
        return 16 + extra_cycles;
    } else {
        return 8;
    }
}

fn perform_return(cpu: &mut CPU, memory: &mut [u8]) {
    let stack_pointer = cpu.read_16_bits(RegisterLabel16::StackPointer);

    // Get the top 2 bytes of the stack
    let lower_byte = memory[stack_pointer as usize];
    let higher_byte = memory[stack_pointer as usize + 1];

    // Move the stack pointer
    cpu.write_16_bits(RegisterLabel16::StackPointer, stack_pointer + 2);

    // Set the program counter to the value from the stack
    let new_program_counter = u16::from_be_bytes([higher_byte, lower_byte]);

    cpu.write_16_bits(RegisterLabel16::ProgramCounter, new_program_counter);
}
