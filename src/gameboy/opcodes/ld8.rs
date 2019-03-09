use super::ReadWriteRegister;
use super::{Argument, OpCode};

impl OpCode {
    pub fn run_ld8<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        memory: &mut Vec<u8>,
    ) -> u32 {
        let mut cycles = 0;
        assert_eq!(self.args.len(), 2);
        {
            let source = match self.args[1] {
                Argument::Register8Constant(register) => cpu.read_8_bits(register),
                Argument::RegisterIndirect(register) => {
                    cycles += 4;
                    memory[cpu.read_16_bits(register) as usize]
                }
                Argument::SmallValue(val) => val,
                _ => panic!("Command does not support argument {:?}", self.args[1]),
            };

            let mut dest = |val: u8| match self.args[0] {
                Argument::RegisterIndirectDec(register) => {
                    cycles += 4;
                    memory[cpu.read_16_bits(register) as usize] = val;
                }
                Argument::RegisterIndirect(register) => {
                    let address = cpu.read_16_bits(register);
                    memory[address as usize] = val;
                    cycles += 4;
                }
                Argument::HighOffsetConstant(offset) => {
                    let address = (0xFF00 as usize) + (offset as usize);
                    memory[address] = val;
                    cycles += 8;
                }
                Argument::Register8Constant(register) => {
                    cpu.write_8_bits(register, val);
                }
                Argument::HighOffsetRegister(register) => {
                    cycles += 4;
                    let offset = cpu.read_8_bits(register) as u16;
                    memory[(0xFF00 + offset) as usize] = val;
                }
                _ => panic!("Command does not support argument {:?}", self.args[0]),
            };

            dest(source);
            cycles += 4;
        }

        match self.args[0] {
            Argument::RegisterIndirectDec(register) => {
                let new_val = cpu.read_16_bits(register) - 1;
                cpu.write_16_bits(register, new_val);
            }
            _ => {} // Do nothing
        }

        cycles
    }
}
