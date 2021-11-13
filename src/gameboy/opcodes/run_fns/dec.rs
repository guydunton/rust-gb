use crate::gameboy::cpu::CPU;

use super::super::super::{write_flag, Flags};
use super::super::Argument;

pub fn run_dec(args: &[Argument], cpu: &mut CPU, _memory: &mut Vec<u8>) -> u32 {
    // Reset Z & H flags flags. Ignore N because it's always set to 1
    write_flag(cpu, Flags::Z, false);
    write_flag(cpu, Flags::H, false);

    match args[0] {
        Argument::Register8Constant(register) => {
            // Get the value in the register
            let b = cpu.read_8_bits(register);

            // If the result will be 0 then set the Z flag
            if b == 1 {
                write_flag(cpu, Flags::Z, true);
            }

            // If result borrows from top half of byte set H flag
            if b == 0b1_000 {
                write_flag(cpu, Flags::H, true);
            }

            // Always set the N flag to 1
            write_flag(cpu, Flags::N, true);

            // Reduce and write back to register
            cpu.write_8_bits(register, b.wrapping_sub(1));

            4
        }
        Argument::Register16Constant(register) => {
            let val = cpu.read_16_bits(register);

            if val > 0 {
                cpu.write_16_bits(register, val - 1);
            }

            8
        }
        _ => panic!("Unknown argument found in DEC opcode"),
    }
}
