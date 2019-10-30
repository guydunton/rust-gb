use super::super::{read_flag, Flags};
use super::argument::JumpCondition;
use super::{Argument, OpCode, ReadWriteRegister, RegisterLabel16};

impl OpCode {
    pub fn run_jmp<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _memory: &mut Vec<u8>,
    ) -> u32 {
        let mut cycles = 0;
        assert_eq!(self.args.len(), 2);

        // 8 cycles by default
        cycles += 8;

        // Arg 1 is the condition
        let condition = match self.args[0] {
            Argument::JumpArgument(condition) => condition,
            _ => panic!("Invalid argument for jump statement {:?}", self.args[0]),
        };

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

            let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
            cpu.write_16_bits(
                RegisterLabel16::ProgramCounter,
                (i32::from(program_counter) + i32::from(distance)) as u16,
            );

            cycles += 4;
        }
        cycles
    }
}
