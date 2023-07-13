use crate::gameboy::cpu::CPU;

use super::super::super::{read_flag, Flags};
use super::super::argument::JumpCondition;
use super::super::Argument;
use crate::gameboy::RegisterLabel16;

pub fn run_jmp(args: &[Argument], cpu: &mut CPU, _memory: &mut [u8]) -> u32 {
    assert!(args.len() <= 2);

    let should_jump = match args[0] {
        Argument::JumpCondition(condition) => {
            let zero_flag = read_flag(cpu, Flags::Z);
            match condition {
                JumpCondition::NotZero => !zero_flag,
                JumpCondition::Zero => zero_flag,
            }
        }
        _ => true,
    };

    // Flags to later calculate cycles
    let mut relative_location = false;
    let mut address_location = false;

    let mut arg_to_location = |arg: Argument| match arg {
        Argument::JumpDistance(distance) => {
            relative_location = true;
            let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
            (i32::from(program_counter) + i32::from(distance)) as u16
        }
        Argument::Label(location) => {
            address_location = true;
            location
        }
        Argument::RegisterIndirect(register) => cpu.read_16_bits(register),
        _ => panic!("Invalid argument for jump statement {:?}", arg),
    };

    let location = match args[0] {
        Argument::JumpCondition(_) => arg_to_location(args[1]),
        _ => arg_to_location(args[0]),
    };

    if should_jump {
        cpu.write_16_bits(RegisterLabel16::ProgramCounter, location);
    }

    // Work out the cycles taken
    if address_location && should_jump {
        16 // a16 jump is 16
    } else if address_location && !should_jump {
        12 // a16 no jump is 12
    } else if relative_location && should_jump {
        12 // r8 jump is 12
    } else if relative_location && !should_jump {
        8 // r8 no jump is 8
    } else {
        4 // (hl) is 4
    }
}
