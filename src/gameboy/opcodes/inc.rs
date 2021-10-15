use crate::gameboy::cpu::CPU;

use super::super::{write_flag, Flags};
use super::{Argument, OpCode};

impl OpCode {
    pub fn run_inc(&self, cpu: &mut CPU, _memory: &mut Vec<u8>) -> u32 {
        match self.args[0] {
            Argument::Register8Constant(reg) => {
                let reg_value = cpu.read_8_bits(reg);

                let (new_val, overflow) = reg_value.overflowing_add(1);

                if overflow {
                    write_flag(cpu, Flags::Z, true);
                }

                write_flag(cpu, Flags::N, false);

                if new_val == (0x0F + 1) {
                    write_flag(cpu, Flags::H, true);
                }

                cpu.write_8_bits(reg, new_val);
            }
            Argument::Register16Constant(register) => {
                let current_value = cpu.read_16_bits(register);
                let (result, _) = current_value.overflowing_add(1);
                cpu.write_16_bits(register, result);
            }
            _ => {
                panic!(
                    "Unsupported argument found in IN instruction: {:?}",
                    self.args[0]
                );
            }
        }
        get_argument_cycles(self.args[0])
    }
}

fn get_argument_cycles(argument: Argument) -> u32 {
    match argument {
        Argument::Register8Constant(_) => 4,
        Argument::Register16Constant(_) => 8,
        _ => 0,
    }
}
