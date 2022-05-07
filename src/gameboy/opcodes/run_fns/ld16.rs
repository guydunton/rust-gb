use crate::gameboy::{cpu::CPU, write_flag, Flags, RegisterLabel16, RegisterLabel8};

use super::super::Argument;

pub fn run_ld16(args: &[Argument], cpu: &mut CPU, memory: &mut [u8]) -> u32 {
    assert_eq!(args.len(), 2);

    let source = match args[1] {
        Argument::LargeValue(val) => val,
        Argument::Register16Constant(register) => cpu.read_16_bits(register),
        Argument::SPOffset(offset) => {
            let sp = cpu.read_16_bits(RegisterLabel16::StackPointer) as i32;
            let result = sp + (offset as i32);

            // Reset the flags
            cpu.write_8_bits(RegisterLabel8::F, 0);
            if sp <= u8::MAX as i32 && result > u8::MAX as i32 {
                write_flag(cpu, Flags::C, true);
            }
            if sp <= 0xFu8 as i32 && result > 0xFu8 as i32 {
                write_flag(cpu, Flags::H, true);
            }
            result as u16
        }
        _ => panic!("Command does not support argument {:?}", args[1]),
    };

    let mut dest = |val: u16| match args[0] {
        Argument::Register16Constant(register) => cpu.write_16_bits(register, val),
        Argument::AddressIndirect(address) => {
            let [ls_byte, ms_byte] = val.to_le_bytes();
            memory[address as usize] = ls_byte;
            memory[(address + 1) as usize] = ms_byte;
        }
        _ => panic!("Command does not support argument {:?}", args[0]),
    };

    dest(source);

    let mut cycles = 8;

    cycles += match args[0] {
        Argument::AddressIndirect(_) => 12,
        _ => 0,
    };

    cycles += match args[1] {
        Argument::LargeValue(_) => 4,
        Argument::SPOffset(_) => 4,
        _ => 0,
    };

    cycles
}
