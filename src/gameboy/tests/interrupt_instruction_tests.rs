use crate::gameboy::cpu::CPU;
use crate::gameboy::memory_adapter::MemoryAdapter;
use crate::gameboy::opcodes::{Argument, Category, Decoder};
use crate::gameboy::tests::decode_util::decode;
use crate::gameboy::{Gameboy, Labels, OpCode, RegisterLabel16, RegisterLabel8};

#[test]
fn decode_interrupt_instructions() {
    let decode = |memory| Decoder::decode_instruction(0x00, memory).unwrap();

    assert_eq!(
        decode(&[0xFB]),
        OpCode::new(Category::EI, [Argument::None, Argument::None])
    );
    assert_eq!(
        decode(&[0xF3]),
        OpCode::new(Category::DI, [Argument::None, Argument::None])
    );
}

#[test]
fn ei_instruction_works() {
    let opcode = OpCode::new(Category::EI, [Argument::None, Argument::None]);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 4);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 1);

    assert_eq!(cpu.is_interrupt_enable_started(), true);
    assert_eq!(cpu.is_interrupts_enabled(), false);
}

#[test]
fn di_instruction_works() {
    let opcode = OpCode::new(Category::DI, [Argument::None, Argument::None]);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.enable_interrupts();

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 4);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 1);

    assert_eq!(cpu.is_interrupt_enable_started(), false);
    assert_eq!(cpu.is_interrupts_enabled(), false);
}

#[test]
fn interrupt_is_only_enabled_after_instruction_after_ei() {
    // EI, NOP, LD A 0x01
    let mut gb = Gameboy::new(vec![0xFB, 0x00, 0x3E, 0x01]);

    // Setup the stack
    gb.set_register_16(RegisterLabel16::StackPointer, 0xC055);

    // Enable VBlank interrupt
    gb.set_memory_at(0xFFFF, 0b0000_0001);

    // Trigger an interrupt
    gb.set_memory_at(Labels::INTERRUPT_TRIGGER, 0b0000_0001);

    // Enable interrupts
    gb.step_once();
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

    // Interrupts aren't enabled until after the next instruction
    assert_eq!(gb.get_ime_flag(), false);

    // Run NOP instructions
    gb.step_once();
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);

    // Running the next instruction triggers the interrupt and we jump to the vblank routine
    let cycles = gb.step_once().unwrap();
    assert_eq!(cycles, 20);

    // The stack contains the return address
    assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0xC053);
    assert_eq!(gb.get_memory_at(0xC054), 0x00);
    assert_eq!(gb.get_memory_at(0xC053), 0x02);

    // The program is now in the VBlank interrupt
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0040);

    // Interrupts are also disabled automatically
    assert_eq!(gb.get_ime_flag(), false);

    // The LD A 0x01 instruction is not run
    assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x0);

    // The vblank trigger has been reset
    assert!((gb.get_memory_at(Labels::INTERRUPT_TRIGGER) & 0b0000_0001) == 0);
}

#[test]
fn decode_rst_instructions() {
    let rst = |address| OpCode::new(Category::RST, [Argument::Vector(address), Argument::None]);

    assert_eq!(decode(&[0xC7]), rst(0x00));
    assert_eq!(decode(&[0xD7]), rst(0x10));
    assert_eq!(decode(&[0xE7]), rst(0x20));
    assert_eq!(decode(&[0xF7]), rst(0x30));
    assert_eq!(decode(&[0xCF]), rst(0x08));
    assert_eq!(decode(&[0xDF]), rst(0x18));
    assert_eq!(decode(&[0xEF]), rst(0x28));
    assert_eq!(decode(&[0xFF]), rst(0x38));
}

#[test]
fn rst_works_like_call() {
    let opcode = OpCode::new(Category::RST, [Argument::Vector(0x20), Argument::None]);

    let mut cpu = CPU::new();
    let mut memory = vec![0x00; 0xFFFF];

    cpu.write_16_bits(RegisterLabel16::StackPointer, 0xA00F);
    cpu.write_16_bits(RegisterLabel16::ProgramCounter, 0x0002);
    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 16);
    assert_eq!(memory[0xA00E], 0x00);
    assert_eq!(memory[0xA00D], 0x03);

    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x0020);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::StackPointer), 0xA00D);
}
