use crate::gameboy::cpu::CPU;
use crate::gameboy::memory_adapter::MemoryAdapter;

use super::super::super::{write_flag, Flags};
use super::super::Argument;

pub fn run_inc(args: &[Argument], cpu: &mut CPU, memory: &mut MemoryAdapter) -> u32 {
    match args[0] {
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
        Argument::RegisterIndirect(register) => {
            let address = cpu.read_16_bits(register);
            let value = memory.get_memory_at(address);

            let (new_val, overflow) = value.overflowing_add(1);

            if overflow {
                write_flag(cpu, Flags::Z, true);
            }

            write_flag(cpu, Flags::N, false);

            if new_val == (0x0F + 1) {
                write_flag(cpu, Flags::H, true);
            }

            memory.set_memory_at(address, new_val);
        }
        _ => {
            panic!(
                "Unsupported argument found in INC instruction: {:?}",
                args[0]
            );
        }
    }
    get_argument_cycles(args[0])
}

fn get_argument_cycles(argument: Argument) -> u32 {
    match argument {
        Argument::Register8Constant(_) => 4,
        Argument::Register16Constant(_) => 8,
        Argument::RegisterIndirect(_) => 12,
        _ => 0,
    }
}
