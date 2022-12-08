use crate::gameboy::{cpu::CPU, write_flag, Flags, RegisterLabel16};

// use super::super::super::flags_register::{write_flag, Flags};
use super::super::argument::Argument;

pub fn run_add16(args: &[Argument], cpu: &mut CPU, _: &mut [u8]) -> u32 {
    let left_val = match args[0] {
        Argument::Register16Constant(register) => cpu.read_16_bits(register),
        _ => {
            panic!("Unknown left argument {:?} in ADD16", args[0]);
        }
    };

    let right_val = match args[1] {
        Argument::Register16Constant(register) => cpu.read_16_bits(register),
        _ => {
            panic!("Unknown right argument {:?} in ADD16", args[1]);
        }
    };

    let (result, overflowed) = left_val.overflowing_add(right_val);

    // If overflowed 11th bit
    if left_val <= 0b0000_1111_1111_1111 && result >= 0b0001_0000_0000_0000 {
        write_flag(cpu, Flags::H, true);
    }

    if overflowed {
        write_flag(cpu, Flags::C, true);
    }

    write_flag(cpu, Flags::N, false);

    cpu.write_16_bits(RegisterLabel16::HL, result);

    8
}
