use crate::gameboy::cpu::CPU;

use super::super::super::flags_register::{Flags, write_flag};
use super::super::argument::Argument;

pub fn run_bit(args: &[Argument], cpu: &mut CPU, memory: &mut [u8]) -> u32 {
    let mut cycles = 0;
    assert_eq!(args.len(), 2);

    match (args[0], args[1]) {
        (Argument::Bit(bit), Argument::Register8Constant(register)) => {
            let register = cpu.read_8_bits(register);

            let result = (((0x1 << bit) ^ register) >> bit) == 1;
            write_flag(cpu, Flags::Z, result);
            write_flag(cpu, Flags::N, false);
            write_flag(cpu, Flags::H, true);
            cycles += 8;
        }
        (Argument::Bit(bit), Argument::RegisterIndirect(register)) => {
            let address = cpu.read_16_bits(register) as usize;
            let value = memory[address];

            let result = (((0x1 << bit) ^ value) >> bit) == 1;
            write_flag(cpu, Flags::Z, result);
            write_flag(cpu, Flags::N, false);
            write_flag(cpu, Flags::H, true);
            cycles += 12;
        }
        _ => panic!("Invalid arguments"),
    }

    cycles
}
