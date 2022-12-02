use crate::gameboy::cpu::CPU;

use super::super::argument::Argument;

pub fn run_ei(_args: &[Argument], cpu: &mut CPU, _memory: &mut [u8]) -> u32 {
    cpu.enable_global_interrupt();
    4
}

pub fn run_di(_args: &[Argument], cpu: &mut CPU, _memory: &mut [u8]) -> u32 {
    cpu.disable_interrupts();
    4
}
