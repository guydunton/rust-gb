use crate::{
    gameboy::{
        cpu::CPU,
        memory_adapter::MemoryAdapter,
        opcodes::{Argument, Category},
        tests::decode_util::decode,
        Flags, OpCode, RegisterLabel16, RegisterLabel8,
    },
    Gameboy,
};

fn add_fixture_gb<'a>(code: u8, a_val: u8, source_val: u8) -> Gameboy<'a> {
    let mut gb = Gameboy::new(vec![code]);
    gb.set_register_8(RegisterLabel8::A, a_val);
    gb.set_register_16(RegisterLabel16::HL, 0x4000);
    gb.set_memory_at(0x4000, source_val);
    gb
}

#[test]
fn add_hl_size_test() {
    let mut gb = add_fixture_gb(0x86, 1, 2);

    let cycles = gb.step_once();

    assert_eq!(cycles, 8);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

    // The result is written back to A
    assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x03);

    // ADD A (HL) sets the N flag to 0
    assert!(!gb.get_flag(Flags::N));
    assert!(!gb.get_flag(Flags::Z));
}

#[test]
fn add_hl_sets_z_if_result_0() {
    let mut gb = add_fixture_gb(0x86, 0, 0);
    gb.step_once();

    assert!(gb.get_flag(Flags::Z));
}

#[test]
fn add_hl_h_flag_overflow_at_half_byte() {
    let mut gb = add_fixture_gb(0x86, 0b0000_1111, 1);
    gb.step_once();

    assert!(gb.get_flag(Flags::H));
}

#[test]
fn add_hl_c_flag_overflow_byte() {
    let mut gb = add_fixture_gb(0x86, 0b1111_1111, 1);
    gb.step_once();
    assert!(gb.get_flag(Flags::C));
}

// Reset the flags is they are already set
#[test]
fn add_hl_reset_flags_if_set() {
    let mut gb = add_fixture_gb(0x86, 1, 1);
    gb.set_flag(Flags::Z, true);
    gb.set_flag(Flags::H, true);
    gb.set_flag(Flags::N, true);
    gb.set_flag(Flags::C, true);
    gb.step_once();

    assert!(!gb.get_flag(Flags::Z));
    assert!(!gb.get_flag(Flags::H));
    assert!(!gb.get_flag(Flags::N));
    assert!(!gb.get_flag(Flags::C));
}

#[test]
fn add_r8_to_a() {
    let opcode = OpCode::new(
        Category::ADD,
        [
            Argument::Register8Constant(RegisterLabel8::A),
            Argument::Register8Constant(RegisterLabel8::B),
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0x02);
    cpu.write_8_bits(RegisterLabel8::B, 0x02);

    let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x04);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
    assert_eq!(cycles, 4);
}

#[test]
fn add_d8_to_a() {
    let opcode = OpCode::new(
        Category::ADD,
        [
            Argument::Register8Constant(RegisterLabel8::A),
            Argument::SmallValue(0x04),
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0x02);

    let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x06);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x02);
    assert_eq!(cycles, 8);
}

#[test]
fn decode_add_instructions() {
    let add_a_r8 = |register| {
        OpCode::new(
            Category::ADD,
            [
                Argument::Register8Constant(RegisterLabel8::A),
                Argument::Register8Constant(register),
            ],
        )
    };

    const A: RegisterLabel8 = RegisterLabel8::A;
    const B: RegisterLabel8 = RegisterLabel8::B;
    const C: RegisterLabel8 = RegisterLabel8::C;
    const D: RegisterLabel8 = RegisterLabel8::D;
    const E: RegisterLabel8 = RegisterLabel8::E;
    const H: RegisterLabel8 = RegisterLabel8::H;
    const L: RegisterLabel8 = RegisterLabel8::L;

    assert_eq!(decode(&[0x80]), add_a_r8(B));
    assert_eq!(decode(&[0x81]), add_a_r8(C));
    assert_eq!(decode(&[0x82]), add_a_r8(D));
    assert_eq!(decode(&[0x83]), add_a_r8(E));
    assert_eq!(decode(&[0x84]), add_a_r8(H));
    assert_eq!(decode(&[0x85]), add_a_r8(L));
    assert_eq!(decode(&[0x87]), add_a_r8(A));

    assert_eq!(
        decode(&[0xC6, 0x04]),
        OpCode::new(
            Category::ADD,
            [
                Argument::Register8Constant(RegisterLabel8::A),
                Argument::SmallValue(0x04)
            ]
        )
    );
}
