use super::super::Argument;
use crate::gameboy::cpu::CPU;
use crate::gameboy::memory_adapter::MemoryAdapter;

pub fn run_ld8(args: &[Argument], cpu: &mut CPU, memory: &mut MemoryAdapter) -> u32 {
    assert_eq!(args.len(), 2);
    {
        let source = match args[1] {
            Argument::Register8Constant(register) => cpu.read_8_bits(register),
            Argument::RegisterIndirect(register) => {
                memory.get_memory_at(cpu.read_16_bits(register))
            }
            Argument::HighOffsetConstant(offset) => memory.get_memory_at(0xFF00 + offset as u16),
            Argument::SmallValue(val) => val,
            Argument::RegisterIndirectInc(register) => {
                memory.get_memory_at(cpu.read_16_bits(register))
            }
            Argument::RegisterIndirectDec(register) => {
                memory.get_memory_at(cpu.read_16_bits(register))
            }
            Argument::AddressIndirect(address) => memory.get_memory_at(address),
            _ => panic!("Command does not support source argument {:?}", args[1]),
        };

        let mut dest = |val: u8| match args[0] {
            Argument::RegisterIndirectDec(register) => {
                memory.set_memory_at(cpu.read_16_bits(register), val);
            }
            Argument::RegisterIndirectInc(register) => {
                memory.set_memory_at(cpu.read_16_bits(register), val);
            }
            Argument::RegisterIndirect(register) => {
                memory.set_memory_at(cpu.read_16_bits(register), val);
            }
            Argument::HighOffsetConstant(offset) => {
                memory.set_memory_at(0xFF00 + offset as u16, val);
            }
            Argument::Register8Constant(register) => {
                cpu.write_8_bits(register, val);
            }
            Argument::AddressIndirect(address) => {
                memory.set_memory_at(address, val);
            }
            Argument::HighOffsetRegister(register) => {
                memory.set_memory_at(0xFF00 + cpu.read_8_bits(register) as u16, val);
            }
            _ => panic!(
                "Command does not support destination argument {:?}",
                args[0]
            ),
        };

        dest(source);
    }

    for arg in args {
        match arg {
            Argument::RegisterIndirectDec(register) => {
                let new_val = cpu.read_16_bits(*register) - 1;
                cpu.write_16_bits(*register, new_val);
            }
            Argument::RegisterIndirectInc(register) => {
                let new_val = cpu.read_16_bits(*register) + 1;
                cpu.write_16_bits(*register, new_val);
            }
            _ => {} // Do nothing
        }
    }

    // Get the cycle cost of each argument + the base for the command
    4 + get_argument_cycles(args[1]) + get_argument_cycles(args[0])
}

fn get_argument_cycles(argument: Argument) -> u32 {
    match argument {
        Argument::AddressIndirect(_) => 12,
        Argument::RegisterIndirect(_) => 4,
        Argument::RegisterIndirectDec(_) => 4,
        Argument::RegisterIndirectInc(_) => 4,
        Argument::HighOffsetConstant(_) => 8,
        Argument::HighOffsetRegister(_) => 4,
        Argument::SmallValue(_) => 4,
        _ => 0,
    }
}
