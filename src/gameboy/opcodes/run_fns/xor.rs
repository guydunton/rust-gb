use flags_register::Flags;

use crate::gameboy::cpu::CPU;
use crate::gameboy::flags_register;

use super::super::Argument;
use crate::gameboy::RegisterLabel8;

pub fn run_xor(args: &[Argument], cpu: &mut CPU, _memory: &mut [u8]) -> u32 {
    let mut cycles = 0;

    match args[0] {
        Argument::Register8Constant(register) => {
            let new_val = cpu.read_8_bits(RegisterLabel8::A) ^ cpu.read_8_bits(register);
            cpu.write_8_bits(RegisterLabel8::A, new_val);
            cpu.write_8_bits(RegisterLabel8::F, 0);

            if new_val == 0 {
                flags_register::write_flag(cpu, Flags::Z, true);
            }
        }
        _ => panic!("Argument not supported: {:?}", args[0]),
    }

    cycles += 4;
    cycles
}
