use crate::gameboy::{cpu::CPU, write_flag, Flags, RegisterLabel8};

use super::super::Argument;

#[allow(clippy::eq_op)]
pub fn run_cpl(_: &[Argument], cpu: &mut CPU, _: &mut Vec<u8>) -> u32 {
    let a = cpu.read_8_bits(RegisterLabel8::A);

    cpu.write_8_bits(RegisterLabel8::A, !a);

    write_flag(cpu, Flags::N, true);
    write_flag(cpu, Flags::H, true);

    4
}
