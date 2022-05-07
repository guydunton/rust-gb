use crate::gameboy::{cpu::CPU, RegisterLabel8};

use super::super::super::flags_register::{write_flag, Flags};
use super::super::argument::Argument;

pub fn run_and(args: &[Argument], cpu: &mut CPU, memory: &mut [u8]) -> u32 {
    cpu.write_8_bits(RegisterLabel8::F, 0);
    write_flag(cpu, Flags::H, true);

    match args[0] {
        Argument::SmallValue(val) => {
            let new_val = cpu.read_8_bits(RegisterLabel8::A) & val;
            cpu.write_8_bits(RegisterLabel8::A, new_val);
        }
        Argument::Register8Constant(reg) => {
            let new_val = cpu.read_8_bits(RegisterLabel8::A) & cpu.read_8_bits(reg);
            cpu.write_8_bits(RegisterLabel8::A, new_val);
        }
        Argument::RegisterIndirect(reg) => {
            let address = cpu.read_16_bits(reg);
            let comparitor = memory[address as usize];

            let new_val = cpu.read_8_bits(RegisterLabel8::A) & comparitor;
            cpu.write_8_bits(RegisterLabel8::A, new_val);
        }
        _ => panic!("Unknown AND argument {:?}", args[0]),
    }

    if cpu.read_8_bits(RegisterLabel8::A) == 0 {
        write_flag(cpu, Flags::Z, true);
    }

    8
}
