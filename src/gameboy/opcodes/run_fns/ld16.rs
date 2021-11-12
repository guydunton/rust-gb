use crate::gameboy::cpu::CPU;

use super::super::Argument;

pub fn run_ld16(args: &[Argument], cpu: &mut CPU, _: &mut Vec<u8>) -> u32 {
    assert_eq!(args.len(), 2);

    let mut dest = |val: u16| match args[0] {
        Argument::Register16Constant(register) => cpu.write_16_bits(register, val),
        _ => panic!("Command does not support argument {:?}", args[0]),
    };

    let source = || match args[1] {
        Argument::LargeValue(val) => val,
        _ => panic!("Command does not support argument {:?}", args[1]),
    };

    dest(source());

    return 12;
}
