use crate::gameboy::{cpu::CPU, write_flag, Flags};

pub fn run_scf(cpu: &mut CPU, _memory: &mut [u8]) -> u32 {
    write_flag(cpu, Flags::C, true);
    write_flag(cpu, Flags::N, false);
    write_flag(cpu, Flags::H, false);
    4
}
