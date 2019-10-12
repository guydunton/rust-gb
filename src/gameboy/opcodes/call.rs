use super::argument::Argument;
use super::OpCode;
use super::ReadWriteRegister;
use crate::gameboy::RegisterLabel16;

impl OpCode {
    pub fn run_call<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        memory: &mut Vec<u8>,
    ) -> u32 {
        let mut cycles = 0;
        if let Argument::Label(address) = self.args[0] {
            // Store the contents of the program counter on the stack
            let pc = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
            let return_address = pc.to_be_bytes();

            let stack = cpu.read_16_bits(RegisterLabel16::StackPointer);
            memory[(stack - 1) as usize] = return_address[0];
            memory[(stack - 2) as usize] = return_address[1];

            // Update the stack
            cpu.write_16_bits(RegisterLabel16::StackPointer, stack - 2);

            // Move the program counter to the value of the argument
            cpu.write_16_bits(RegisterLabel16::ProgramCounter, address);

            cycles += 24;
        }
        cycles
    }
}
