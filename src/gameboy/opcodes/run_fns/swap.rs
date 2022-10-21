use crate::gameboy::cpu::CPU;
use crate::gameboy::RegisterLabel8;

use super::super::super::flags_register::{write_flag, Flags};
use super::super::argument::Argument;

pub fn run_swap(args: &[Argument], cpu: &mut CPU, memory: &mut [u8]) -> u32 {
    let mut cycles = 0;
    assert_eq!(args.len(), 2);

    let zero_result;

    match args[0] {
        Argument::Register8Constant(register) => {
            // Swap the top & bottom nibbles
            let value = cpu.read_8_bits(register);

            let top_nibble = value & 0b1111_0000;
            let result = (value << 4) | (top_nibble >> 4);

            zero_result = result == 0;

            cpu.write_8_bits(register, result);
        }
        Argument::RegisterIndirect(register) => {
            let address = cpu.read_16_bits(register) as usize;
            let value = memory[address];

            let top_nibble = value & 0b1111_0000;
            let result = (value << 4) | (top_nibble >> 4);

            zero_result = result == 0;

            memory[address] = result;

            cycles += 8;
        }
        _ => panic!("Invalid arguments"),
    }

    // Set all flags to zero & set Z flag based on result
    cpu.write_8_bits(RegisterLabel8::F, 0);
    write_flag(cpu, Flags::Z, zero_result);

    cycles += 8;
    cycles
}
