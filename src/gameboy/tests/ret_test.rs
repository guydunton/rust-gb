use crate::gameboy::cpu::CPU;
use crate::gameboy::memory_adapter::MemoryAdapter;
use crate::gameboy::opcodes::Argument;
use crate::gameboy::opcodes::Category;
use crate::gameboy::opcodes::JumpCondition;
use crate::gameboy::tests::decode_util::decode;
use crate::gameboy::write_flag;
use crate::gameboy::Flags;
use crate::gameboy::Gameboy;
use crate::gameboy::OpCode;
use crate::gameboy::RegisterLabel16;

#[test]
fn ret_jumps_back_to_correct_place() {
    // 0x00, 0x01, 0x02
    let mut gb = Gameboy::new(vec![0xC9, 0x34, 0x12]); // RET

    // 0x00 : (0xC9) The RET instruction
    // 0x01 : (0x34) The lower byte of the return address
    // 0x02 : (0x12) The higher byte of the return address

    // The stack pointer points to 0x01 which means the value
    // at the top of the stack is 0x1234
    gb.set_register_16(RegisterLabel16::StackPointer, 0x01);

    let cycles = gb.step_once().unwrap();

    // RET takes 16 cycles
    assert_eq!(cycles, 16);

    // Program counter is now in the correct place
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1234);

    // Stack pointer is also in the right place
    assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0x03);
}

#[test]
fn run_instruction_is_1_byte_big() {
    use crate::gameboy::opcodes::Decoder;
    let instructions = vec![0xC9];
    let opcode = Decoder::decode_instruction(0x00, &instructions).unwrap();
    assert_eq!(opcode.size(), 1);
}

#[test]
fn decode_ret_instructions() {
    assert_eq!(
        decode(&[0xC0]),
        OpCode::new(
            Category::RET,
            [
                Argument::JumpCondition(JumpCondition::NotZero),
                Argument::None
            ]
        )
    );
    assert_eq!(
        decode(&[0xC8]),
        OpCode::new(
            Category::RET,
            [Argument::JumpCondition(JumpCondition::Zero), Argument::None]
        )
    );
    assert_eq!(
        decode(&[0xD0]),
        OpCode::new(
            Category::RET,
            [
                Argument::JumpCondition(JumpCondition::NotCarry),
                Argument::None
            ]
        )
    );
    assert_eq!(
        decode(&[0xD8]),
        OpCode::new(
            Category::RET,
            [
                Argument::JumpCondition(JumpCondition::Carry),
                Argument::None
            ]
        )
    );
}

#[test]
fn ret_can_have_nz_check() {
    let opcode = OpCode::new(
        Category::RET,
        [
            Argument::JumpCondition(JumpCondition::NotZero),
            Argument::None,
        ],
    );

    let mut memory = vec![0x00; 0xFF];
    memory[0x01] = 0x34;
    memory[0x02] = 0x12;

    let mut cpu = CPU::new();

    cpu.write_16_bits(RegisterLabel16::StackPointer, 0x01);
    write_flag(&mut cpu, Flags::Z, true);

    // Don't jump of zero flag is true
    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
    assert_eq!(cycles, 8);

    // Jump if zero flag is false
    memory[0x01] = 0x34;
    memory[0x02] = 0x12;
    cpu.write_16_bits(RegisterLabel16::StackPointer, 0x01);
    write_flag(&mut cpu, Flags::Z, false);

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x1234);
    assert_eq!(cycles, 20);
}

#[test]
fn ret_with_zero_check() {
    let opcode = OpCode::new(
        Category::RET,
        [Argument::JumpCondition(JumpCondition::Zero), Argument::None],
    );

    let mut memory = vec![0x00; 0xFF];
    memory[0x01] = 0x34;
    memory[0x02] = 0x12;

    let mut cpu = CPU::new();

    cpu.write_16_bits(RegisterLabel16::StackPointer, 0x01);
    write_flag(&mut cpu, Flags::Z, true);

    // Jump when zero flag is true
    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x1234);
}

#[test]
fn ret_with_not_carry_check() {
    let opcode = OpCode::new(
        Category::RET,
        [
            Argument::JumpCondition(JumpCondition::NotCarry),
            Argument::None,
        ],
    );

    let mut memory = vec![0x00; 0xFF];
    memory[0x01] = 0x34;
    memory[0x02] = 0x12;

    let mut cpu = CPU::new();

    cpu.write_16_bits(RegisterLabel16::StackPointer, 0x01);
    write_flag(&mut cpu, Flags::C, false);

    // Jump when carry flag is false
    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x1234);
}

#[test]
fn ret_with_carry_check() {
    let opcode = OpCode::new(
        Category::RET,
        [
            Argument::JumpCondition(JumpCondition::Carry),
            Argument::None,
        ],
    );

    let mut memory = vec![0x00; 0xFF];
    memory[0x01] = 0x34;
    memory[0x02] = 0x12;

    let mut cpu = CPU::new();

    cpu.write_16_bits(RegisterLabel16::StackPointer, 0x01);
    write_flag(&mut cpu, Flags::C, true);

    // Jump when carry flag is false
    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x1234);
}

#[test]
fn ret_with_condition_still_1_byte() {
    let opcode = OpCode::new(
        Category::RET,
        [
            Argument::JumpCondition(JumpCondition::NotZero),
            Argument::None,
        ],
    );

    assert_eq!(opcode.size(), 1);
}

#[test]
fn decode_the_reti_instuction() {
    assert_eq!(
        decode(&[0xD9]),
        OpCode::new(Category::RETI, [Argument::None, Argument::None])
    );
}

#[test]
fn reti_returns_and_enables_interrupts() {
    /*
     * RETI
     * DATA: 0x0003 <- Stack points points here
     * NOP <- Interrupts will be enabled after here
     */
    let mut gb = Gameboy::new(vec![0xD9, 0x03, 0x00, 0x00]);
    gb.set_register_16(RegisterLabel16::StackPointer, 0x01);

    assert!(!gb.get_ime_flag()); // Interrupts are initially disabled

    // Run the ret instruction
    let cycles = gb.step_once().unwrap();

    assert_eq!(cycles, 16);

    // Check we returned
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x03);
    assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0x03);

    assert!(!gb.get_ime_flag()); // interrupts aren't enabled yet

    gb.step_once(); // Run the nop to enable interrupts

    assert!(gb.get_ime_flag());
}
