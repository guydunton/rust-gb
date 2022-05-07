use crate::gameboy::{cpu::CPU, RegisterLabel8};

use super::super::super::flags_register::{write_flag, Flags};
use super::super::argument::Argument;

pub fn run_and(args: &[Argument], cpu: &mut CPU, _: &mut Vec<u8>) -> u32 {
    if let Argument::SmallValue(val) = args[0] {
        let new_val = cpu.read_8_bits(RegisterLabel8::A) & val;
        cpu.write_8_bits(RegisterLabel8::A, new_val);
        cpu.write_8_bits(RegisterLabel8::F, 0);

        write_flag(cpu, Flags::H, true);

        if new_val == 0 {
            write_flag(cpu, Flags::Z, true);
        }
    }

    8
}
