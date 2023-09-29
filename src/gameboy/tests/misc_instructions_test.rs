use crate::gameboy::{
    cpu::CPU,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category},
    read_flag,
    tests::decode_util::decode,
    write_flag, Flags, OpCode,
};

#[test]
fn decode_scf_instructon() {
    let opcode = OpCode::new(Category::SCF, [Argument::None, Argument::None]);
    assert_eq!(decode(&[0x37]), opcode);

    assert_eq!(opcode.size(), 1);
}

#[test]
fn scf_sets_the_carry_flag_and_others() {
    let opcode = OpCode::new(Category::SCF, [Argument::None, Argument::None]);

    let mut cpu = CPU::new();
    write_flag(&mut cpu, Flags::N, true);
    write_flag(&mut cpu, Flags::H, true);
    write_flag(&mut cpu, Flags::C, true);

    let mut memory = vec![0x0; 0xFF];

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 4);
    assert_eq!(read_flag(&cpu, Flags::N), false);
    assert_eq!(read_flag(&cpu, Flags::H), false);
    assert_eq!(read_flag(&cpu, Flags::C), true);

    // Reset the flags and try again
    write_flag(&mut cpu, Flags::N, false);
    write_flag(&mut cpu, Flags::H, false);
    write_flag(&mut cpu, Flags::C, false);

    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(read_flag(&cpu, Flags::N), false);
    assert_eq!(read_flag(&cpu, Flags::H), false);
    assert_eq!(read_flag(&cpu, Flags::C), true);
}
