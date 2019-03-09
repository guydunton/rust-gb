use super::{Argument, OpCode, ReadWriteRegister, RegisterLabel16};

impl OpCode {
    pub fn run_push<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        memory: &mut Vec<u8>,
    ) -> u32 {
        let mut cycles = 0;
        if let Argument::Register16Constant(reg) = self.args[0] {
            let value = cpu.read_16_bits(reg);
            let bytes = value.to_be_bytes();

            let sp = cpu.read_16_bits(RegisterLabel16::StackPointer);
            memory[(sp - 1) as usize] = bytes[0];
            memory[(sp - 2) as usize] = bytes[1];

            cpu.write_16_bits(RegisterLabel16::StackPointer, sp - 2);

            cycles += 16;
        }
        cycles
    }
}
