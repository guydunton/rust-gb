use super::ReadWriteRegister;
use super::{Argument, OpCode};

impl OpCode {
    pub fn run_ld8<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        memory: &mut Vec<u8>,
    ) -> u32 {
        assert_eq!(self.args.len(), 2);
        {
            let source = match self.args[1] {
                Argument::Register8Constant(register) => cpu.read_8_bits(register),
                Argument::RegisterIndirect(register) => memory[cpu.read_16_bits(register) as usize],
                Argument::HighOffsetConstant(offset) => {
                    memory[(0xFF00 as usize) + (offset as usize)]
                }
                Argument::SmallValue(val) => val,
                _ => panic!(
                    "Command does not support source argument {:?}",
                    self.args[1]
                ),
            };

            let mut dest = |val: u8| match self.args[0] {
                Argument::RegisterIndirectDec(register) => {
                    memory[cpu.read_16_bits(register) as usize] = val;
                }
                Argument::RegisterIndirectInc(register) => {
                    memory[cpu.read_16_bits(register) as usize] = val;
                }
                Argument::RegisterIndirect(register) => {
                    memory[cpu.read_16_bits(register) as usize] = val;
                }
                Argument::HighOffsetConstant(offset) => {
                    memory[(0xFF00 as usize) + (offset as usize)] = val;
                }
                Argument::Register8Constant(register) => {
                    cpu.write_8_bits(register, val);
                }
                Argument::AddressIndirect(address) => {
                    memory[address as usize] = val;
                }
                Argument::HighOffsetRegister(register) => {
                    memory[(0xFF00 + cpu.read_8_bits(register) as u16) as usize] = val;
                }
                _ => panic!(
                    "Command does not support destination argument {:?}",
                    self.args[0]
                ),
            };

            dest(source);
        }

        match self.args[0] {
            Argument::RegisterIndirectDec(register) => {
                let new_val = cpu.read_16_bits(register) - 1;
                cpu.write_16_bits(register, new_val);
            }
            Argument::RegisterIndirectInc(register) => {
                let new_val = cpu.read_16_bits(register) + 1;
                cpu.write_16_bits(register, new_val);
            }
            _ => {} // Do nothing
        }

        // Get the cycle cost of each argument + the base for the command
        4 + get_argument_cycles(self.args[1]) + get_argument_cycles(self.args[0])
    }
}

fn get_argument_cycles(argument: Argument) -> u32 {
    match argument {
        Argument::AddressIndirect(_) => 12,
        Argument::RegisterIndirect(_) => 4,
        Argument::RegisterIndirectDec(_) => 4,
        Argument::RegisterIndirectInc(_) => 4,
        Argument::HighOffsetConstant(_) => 8,
        Argument::HighOffsetRegister(_) => 4,
        _ => 0,
    }
}
