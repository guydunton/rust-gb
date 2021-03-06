use super::super::{write_flag, Flags};
use super::{Argument, OpCode, ReadWriteRegister, RegisterLabel8};

impl OpCode {
    pub fn run_cp<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _memory: &mut Vec<u8>,
    ) -> u32 {
        // Clear all the flags
        cpu.write_8_bits(RegisterLabel8::F, 0);

        // Get the A value
        let a = cpu.read_8_bits(RegisterLabel8::A);

        // Get the argument
        let arg_val = match self.args[0] {
            Argument::SmallValue(val) => val,
            _ => {
                panic!("Unknown argument in CP instruction");
            }
        };

        // Remove argument from A and check the result
        let result = a.checked_sub(arg_val);

        if let Some(r) = result {
            if a >= 0b0001_0000 && r <= 0b0000_1111 {
                // Set the H flag
                write_flag::<T>(cpu, Flags::H, true);
            }
        }

        if arg_val == a {
            // If the values are the same set the zero flag
            write_flag::<T>(cpu, Flags::Z, true);
        } else if arg_val > a {
            // Set the C flag
            write_flag::<T>(cpu, Flags::C, true);
        }

        // Set the N flag to 1
        write_flag::<T>(cpu, Flags::N, true);

        8
    }
}
