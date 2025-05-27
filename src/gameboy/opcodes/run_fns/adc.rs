use crate::gameboy::{cpu::CPU, read_flag, write_flag, Flags, RegisterLabel8};

use super::super::argument::Argument;

struct FlagResult {
    zero: bool,
    half_carry: bool,
    carry: bool,
}

fn checked_add(v1: u8, v2: u8) -> (u8, FlagResult) {
    // sum = a + b
    // no_carry_sum = a ^ b
    // carry_info = sum ^ no_carry_sum
    // half_carry = carry_info & 0x10
    // carry = carry_info & 0x100

    let wrapped_sum = v1.wrapping_add(v2);
    let sum = v1 as u16 + v2 as u16;
    let no_carry_sum = (v1 ^ v2) as u16;
    let carry_info = sum ^ no_carry_sum;
    let half_carry = ((carry_info & 0x10) >> 4) > 0;
    let carry = ((carry_info & 0x100) >> 8) > 0;

    (
        wrapped_sum,
        FlagResult {
            zero: wrapped_sum == 0,
            half_carry,
            carry,
        },
    )
}

pub fn run_adc(args: &[Argument], cpu: &mut CPU, _memory: &mut [u8]) -> u32 {
    // Result: a = r8 + carry flag + a
    // Z = Z, N = 0, H = H, C = C

    let mut rhs = match args[0] {
        Argument::Register8Constant(register) => cpu.read_8_bits(register),
        _ => panic!("Invalid argument for ADC {:?}", args[0]),
    };

    let mut lhs = cpu.read_8_bits(RegisterLabel8::A);
    let carry = if read_flag(cpu, Flags::C) { 1 } else { 0 };

    if lhs < 0xFF {
        lhs += carry;
    } else if rhs < 0xFF {
        rhs += carry;
    }

    let (mut result, flags) = checked_add(lhs, rhs);

    if lhs == 0xFF && rhs == 0xFF {
        result += carry;
    }

    write_flag(cpu, Flags::N, false);
    write_flag(cpu, Flags::Z, flags.zero);
    write_flag(cpu, Flags::H, flags.half_carry);
    write_flag(cpu, Flags::C, flags.carry);

    cpu.write_8_bits(RegisterLabel8::A, result);

    return 4;
}
