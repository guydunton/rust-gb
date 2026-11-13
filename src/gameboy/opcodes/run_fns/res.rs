use super::super::argument::Argument;
use crate::gameboy::cpu::CPU;
use crate::gameboy::memory_adapter::MemoryAdapter;

fn bit_mask(bit: u8) -> u8 {
    !(1 << bit)
}

pub fn run_res(args: &[Argument], cpu: &mut CPU, memory: &mut MemoryAdapter) -> u32 {
    let mut cycles = 0;
    assert_eq!(args.len(), 2);

    match (args[0], args[1]) {
        (Argument::Bit(bit), Argument::Register8Constant(register)) => {
            let value = cpu.read_8_bits(register);
            cpu.write_8_bits(register, value & bit_mask(bit));
            cycles += 8;
        }
        (Argument::Bit(bit), Argument::RegisterIndirect(register)) => {
            let address = cpu.read_16_bits(register);
            let value = memory.get_memory_at(address);
            memory.set_memory_at(address, value & bit_mask(bit));
            cycles += 16;
        }
        _ => panic!("Invalid arguments"),
    }

    cycles
}
