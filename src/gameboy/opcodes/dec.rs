use super::super::{write_flag, Flags};
use super::{Argument, OpCode, ReadWriteRegister, RegisterLabel8};

impl OpCode {
    pub fn run_dec<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _memory: &mut Vec<u8>,
    ) -> u32 {
        if let Argument::Register8Constant(_) = self.args[0] {
            // Get the value in the register
            let b = cpu.read_8_bits(RegisterLabel8::B);

            // If the result will be 0 then set the Z flag
            if b == 1 {
                write_flag::<T>(cpu, Flags::Z, true);
            }

            // If result borrows from top half of byte set H flag
            if b == 0b1_000 {
                write_flag::<T>(cpu, Flags::H, true);
            }

            // Always set the N flag to 1
            write_flag::<T>(cpu, Flags::N, true);

            // Reduce and write back to register
            cpu.write_8_bits(RegisterLabel8::B, b - 1);
        } else {
            panic!("Unknown argument found in DEC opcode");
        }

        4
    }
}
