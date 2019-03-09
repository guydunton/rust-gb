use super::ReadWriteRegister;
use super::{Argument, OpCode};

impl OpCode {
    pub fn run_ld16<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _: &mut Vec<u8>,
    ) -> u32 {
        assert_eq!(self.args.len(), 2);

        let mut dest = |val: u16| match self.args[0] {
            Argument::Register16Constant(register) => cpu.write_16_bits(register, val),
            _ => panic!("Command does not support argument {:?}", self.args[0]),
        };

        let source = || match self.args[1] {
            Argument::LargeValue(val) => val,
            _ => panic!("Command does not support argument {:?}", self.args[1]),
        };

        dest(source());

        return 12;
    }
}
