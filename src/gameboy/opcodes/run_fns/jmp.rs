use crate::gameboy::cpu::CPU;

use super::super::super::{read_flag, Flags};
use super::super::argument::JumpCondition;
use super::super::Argument;
use crate::gameboy::RegisterLabel16;

fn move_program_counter(cpu: &mut CPU, distance: i8) {
    let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
    cpu.write_16_bits(
        RegisterLabel16::ProgramCounter,
        (i32::from(program_counter) + i32::from(distance)) as u16,
    );
}

pub fn run_jmp(args: &[Argument], cpu: &mut CPU, _memory: &mut [u8]) -> u32 {
    assert!(args.len() <= 2);

    // 8 cycles by default
    let mut cycles = 8;

    match args[0] {
        Argument::JumpCondition(condition) => {
            // Arg 1 is the condition
            let condition_checker = || -> bool {
                let zero_flag = read_flag(cpu, Flags::Z);
                match condition {
                    JumpCondition::NotZero => !zero_flag,
                    JumpCondition::Zero => zero_flag,
                }
            };

            if condition_checker() {
                // Arg 2 is relative location

                let distance = match args[1] {
                    Argument::JumpDistance(distance) => distance,
                    _ => panic!("Invalid argument"),
                };

                move_program_counter(cpu, distance);

                cycles += 4;
            }
        }
        Argument::JumpDistance(distance) => {
            move_program_counter(cpu, distance);
            cycles += 4;
        }
        Argument::Label(location) => {
            cpu.write_16_bits(RegisterLabel16::ProgramCounter, location);
            cycles += 8;
        }
        _ => {
            panic!("Invalid argument for jump statement {:?}", args[0]);
        }
    };

    cycles
}
