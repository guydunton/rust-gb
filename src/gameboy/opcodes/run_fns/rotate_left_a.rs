use crate::gameboy::cpu::CPU;

use super::super::super::{read_flag, write_flag, Flags, RegisterLabel8};
use super::rotate_method::shift_reg_and_flag;

pub fn run_rla(cpu: &mut CPU, _memory: &mut [u8]) -> u32 {
    let mut cycles = 0;
    let reg_contents = cpu.read_8_bits(RegisterLabel8::A);
    let carry_flag = read_flag(cpu, Flags::C);

    // Create the new register value
    let (new_register, new_carry) = shift_reg_and_flag(reg_contents, carry_flag);

    // Set the carry flag
    write_flag(cpu, Flags::C, new_carry);

    // Unset the H & N flags
    write_flag(cpu, Flags::H, false);
    write_flag(cpu, Flags::N, false);

    // Write away the flag
    cpu.write_8_bits(RegisterLabel8::A, new_register);

    // Check the result in the C register for 0 to set the zero flag
    if new_register == 0 {
        write_flag(cpu, Flags::Z, true);
    }

    cycles += 4;
    cycles
}
