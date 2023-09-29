use crate::gameboy::{
    cpu::CPU,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category, Decoder},
    read_flag, Flags, OpCode, RegisterLabel16, RegisterLabel8,
};

#[test]
fn test_cb_decode() {
    let decode = |memory| Decoder::decode_instruction(0x00, memory).unwrap();
    let swap = |register| {
        OpCode::new(
            Category::SWAP,
            [Argument::Register8Constant(register), Argument::None],
        )
    };

    const A: RegisterLabel8 = RegisterLabel8::A;
    const B: RegisterLabel8 = RegisterLabel8::B;
    const C: RegisterLabel8 = RegisterLabel8::C;
    const D: RegisterLabel8 = RegisterLabel8::D;
    const E: RegisterLabel8 = RegisterLabel8::E;
    const H: RegisterLabel8 = RegisterLabel8::H;
    const L: RegisterLabel8 = RegisterLabel8::L;

    assert_eq!(decode(&[0xCB, 0x30]), swap(B));
    assert_eq!(decode(&[0xCB, 0x31]), swap(C));
    assert_eq!(decode(&[0xCB, 0x32]), swap(D));
    assert_eq!(decode(&[0xCB, 0x33]), swap(E));
    assert_eq!(decode(&[0xCB, 0x34]), swap(H));
    assert_eq!(decode(&[0xCB, 0x35]), swap(L));
    assert_eq!(decode(&[0xCB, 0x37]), swap(A));

    assert_eq!(
        decode(&[0xCB, 0x36]),
        OpCode::new(
            Category::SWAP,
            [
                Argument::RegisterIndirect(RegisterLabel16::HL),
                Argument::None
            ]
        )
    );
}

#[test]
fn swap_switches_top_and_bottom_nibbles_of_register() {
    let opcode = OpCode::new(
        Category::SWAP,
        [
            Argument::Register8Constant(RegisterLabel8::A),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0xDE);

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 8);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x1);

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0xED);

    assert_eq!(read_flag(&cpu, Flags::Z), false);
}

#[test]
fn swap_switches_top_and_bottom_nibble_of_offset() {
    let opcode = OpCode::new(
        Category::SWAP,
        [
            Argument::RegisterIndirect(RegisterLabel16::HL),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    memory[0xFF00] = 0xDE;

    cpu.write_16_bits(RegisterLabel16::HL, 0xFF00);

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 16);
    assert_eq!(memory[0xFF00], 0xED);
}

#[test]
fn swap_sets_flags_correctly() {
    let opcode = OpCode::new(
        Category::SWAP,
        [
            Argument::Register8Constant(RegisterLabel8::A),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0x0);
    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(read_flag(&cpu, Flags::Z), true);
    assert_eq!(read_flag(&cpu, Flags::H), false);
    assert_eq!(read_flag(&cpu, Flags::C), false);
    assert_eq!(read_flag(&cpu, Flags::N), false);
}
