use super::super::{read_flag, Flags};
use super::argument::JumpCondition;
use super::{Argument, OpCode, ReadWriteRegister, RegisterLabel16};

fn move_program_counter<T: ReadWriteRegister>(cpu: &mut dyn ReadWriteRegister, distance: i8) {
    let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
    cpu.write_16_bits(
        RegisterLabel16::ProgramCounter,
        (i32::from(program_counter) + i32::from(distance)) as u16,
    );
}

impl OpCode {
    pub fn run_jmp<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _memory: &mut Vec<u8>,
    ) -> u32 {
        assert!(self.args.len() <= 2);

        // 8 cycles by default
        let mut cycles = 8;

        match self.args[0] {
            Argument::JumpArgument(condition) => {
                // Arg 1 is the condition
                let condition_checker = || -> bool {
                    match condition {
                        JumpCondition::NotZero => read_flag::<T>(cpu, Flags::Z) == false,
                        JumpCondition::Zero => read_flag::<T>(cpu, Flags::Z) == true,
                    }
                };

                if condition_checker() {
                    // Arg 2 is relative location

                    let distance = match self.args[1] {
                        Argument::JumpDistance(distance) => distance,
                        _ => panic!("Invalid argument"),
                    };

                    move_program_counter::<T>(cpu, distance);

                    cycles += 4;
                }
                return cycles;
            }
            Argument::JumpDistance(distance) => {
                move_program_counter::<T>(cpu, distance);
                cycles += 4;
                return cycles;
            }
            _ => {
                panic!("Invalid argument for jump statement {:?}", self.args[0]);
            }
        };

    }
}
