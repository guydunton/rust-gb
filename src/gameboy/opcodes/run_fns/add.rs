use crate::gameboy::{cpu::CPU, RegisterLabel8};

use super::super::super::flags_register::{write_flag, Flags};
use super::super::argument::Argument;

pub fn run_add(args: &[Argument], cpu: &mut CPU, memory: &mut Vec<u8>) -> u32 {
    // Reset flags
    cpu.write_8_bits(RegisterLabel8::F, 0);

    let mut extra_cycles = 0;

    let target = match args[0] {
        Argument::Register8Constant(register) => register,
        _ => {
            panic!("Unknown argument to ADD command {}", args[0]);
        }
    };

    let target_value = cpu.read_8_bits(target);

    let source = match args[1] {
        Argument::RegisterIndirect(register) => {
            extra_cycles += 4;
            let address = cpu.read_16_bits(register);
            memory[address as usize]
        }
        _ => {
            panic!("Unknown argument to ADD command {}", args[1]);
        }
    };

    let (result, overflowed) = target_value.overflowing_add(source);

    // If result is 0 set the z flag
    if result == 0 {
        write_flag(cpu, Flags::Z, true);
    }

    // If overflowed
    if overflowed {
        write_flag(cpu, Flags::C, true);
    }

    if result > 0b0000_1111 && target_value < 0b0001_0000 {
        write_flag(cpu, Flags::H, true);
    }

    cpu.write_8_bits(target, result);

    4 + extra_cycles
}
