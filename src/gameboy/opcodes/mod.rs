mod bit;
mod call;
mod inc;
mod jmp;
mod ld16;
mod ld8;
mod push;
mod rotate_left;
mod xor;

use super::flags_register::*;
use super::read_write_register::ReadWriteRegister;
use super::register::{RegisterLabel16, RegisterLabel8};
use std::fmt;

pub fn decode_instruction(program_counter: u16, program_code: &[u8]) -> Result<OpCode, String> {
    let code = program_code[program_counter as usize];

    // Needs to be closure to capture values from memory
    let arg_from_str = |arg: &str| -> Result<Argument, String> {
        match arg {
            "DE" => Ok(Argument::Register16Constant(RegisterLabel16::DE)),
            "HL" => Ok(Argument::Register16Constant(RegisterLabel16::HL)),
            "SP" => Ok(Argument::Register16Constant(RegisterLabel16::StackPointer)),
            "(HL-)" => Ok(Argument::RegisterIndirectDec(RegisterLabel16::HL)),
            "A" => Ok(Argument::Register8Constant(RegisterLabel8::A)),
            "B" => Ok(Argument::Register8Constant(RegisterLabel8::B)),
            "C" => Ok(Argument::Register8Constant(RegisterLabel8::C)),
            "H" => Ok(Argument::Register8Constant(RegisterLabel8::H)),
            "(C)" => Ok(Argument::HighOffsetRegister(RegisterLabel8::C)),
            "(DE)" => Ok(Argument::RegisterIndirect(RegisterLabel16::DE)),
            "(HL)" => Ok(Argument::RegisterIndirect(RegisterLabel16::HL)),
            "(a8)" => Ok(Argument::HighOffsetConstant(
                program_code[program_counter as usize + 1],
            )),
            "a16" => Ok(Argument::Label(u16::from_le_bytes([
                program_code[(program_counter + 1) as usize],
                program_code[(program_counter + 2) as usize],
            ]))),
            "d16" => Ok(Argument::LargeValue(u16::from_le_bytes([
                program_code[(program_counter + 1) as usize],
                program_code[(program_counter + 2) as usize],
            ]))),
            "d8" => Ok(Argument::SmallValue(
                program_code[(program_counter + 1) as usize],
            )),
            "NZ" => Ok(Argument::JumpArgument(JumpCondition::NotZero)),
            "r8" => Ok(Argument::JumpDistance(
                program_code[(program_counter + 1) as usize] as i8,
            )),
            "7" => Ok(Argument::Bit(7)),
            _ => Err(format!("Unknown argument: {}", arg)),
        }
    };

    let opcode = |text: &str| -> Result<OpCode, String> {
        let parts = text.split(' ').collect::<Vec<&str>>();
        let catagory = catagory_from_str(parts[0]);

        let args = parts[1..].iter().map(|arg| arg_from_str(arg));

        let mut clean_args = Vec::new();
        for arg in args {
            clean_args.push(arg?);
        }

        Ok(OpCode::new(catagory, clean_args))
    };

    match code {
        0x00 => opcode("NOP"),
        0x06 => opcode("LD8 B d8"),
        0x0C => opcode("INC C"),
        0x0E => opcode("LD8 C d8"),
        0x11 => opcode("LD16 DE d16"),
        0x1A => opcode("LD8 A (DE)"),
        0x20 => opcode("JR NZ r8"),
        0x21 => opcode("LD16 HL d16"),
        0x31 => opcode("LD16 SP d16"),
        0x32 => opcode("LD8 (HL-) A"),
        0x3E => opcode("LD8 A d8"),
        0x4F => opcode("LD8 C A"),
        0x77 => opcode("LD8 (HL) A"),
        0xAF => opcode("XOR A"),
        0xC5 => Ok(OpCode::new(
            Catagory::PUSH,
            vec![Argument::Register16Constant(RegisterLabel16::BC)],
        )),
        0xCB => {
            // 0xCB is prefix and the next byte shows the actual instruction
            let cb_instruction = program_code[program_counter as usize + 1];
            match cb_instruction {
                0x11 => Ok(OpCode::new(
                    Catagory::RL,
                    vec![Argument::Register8Constant(RegisterLabel8::C)],
                )),
                0x7C => opcode("BIT 7 H"),
                _ => Err(format!("Unknown command 0xCB {:#X}", cb_instruction)),
            }
        }
        0xCD => opcode("CALL a16"),
        0xE0 => opcode("LD8 (a8) A"),
        0xE2 => opcode("LD8 (C) A"),
        _ => Err(format!(
            "Unknown command {:#X} at address: {:#X}",
            code, program_counter
        )),
    }
}

fn catagory_from_str(cat: &str) -> Catagory {
    match cat {
        "NOP" => Catagory::NOP,
        "LD16" => Catagory::LD16,
        "LD8" => Catagory::LD8,
        "XOR" => Catagory::XOR,
        "BIT" => Catagory::BIT,
        "JR" => Catagory::JR,
        "INC" => Catagory::INC,
        "CALL" => Catagory::CALL,
        _ => Catagory::NOP,
    }
}

pub struct OpCode {
    catagory: Catagory,
    args: Vec<Argument>,
}

impl OpCode {
    pub fn run<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        memory: &mut Vec<u8>,
    ) -> u32 {
        // Update the program counter
        let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        cpu.write_16_bits(
            RegisterLabel16::ProgramCounter,
            program_counter + self.size(),
        );

        let mut cycles = 0;

        match self.catagory {
            Catagory::LD16 => {
                cycles += self.run_ld16::<T>(cpu, memory);
            }
            Catagory::LD8 => {
                cycles += self.run_ld8::<T>(cpu, memory);
            }
            Catagory::NOP => {
                // Do nothing
                cycles += 4;
            }
            Catagory::XOR => {
                cycles += self.run_xor::<T>(cpu, memory);
            }
            Catagory::BIT => {
                cycles += self.run_bit::<T>(cpu, memory);
            }
            Catagory::JR => {
                cycles += self.run_jmp::<T>(cpu, memory);
            }
            Catagory::CALL => {
                cycles += self.run_call::<T>(cpu, memory);
            }
            Catagory::PUSH => {
                cycles += self.run_push::<T>(cpu, memory);
            }
            Catagory::INC => {
                cycles += self.run_inc::<T>(cpu, memory);
            }
            Catagory::RL => {
                cycles += self.run_rl::<T>(cpu, memory);
            }
        };

        cycles
    }

    fn new(catagory: Catagory, args: Vec<Argument>) -> OpCode {
        OpCode { catagory, args }
    }

    pub fn size(&self) -> u16 {
        let cb_size = match self.catagory {
            Catagory::BIT => 1,
            Catagory::RL => 1,
            _ => 0,
        };
        self.args
            .iter()
            .map(|arg| match arg {
                Argument::Register8Constant(_) => 0,
                Argument::Register16Constant(_) => 0,
                Argument::RegisterIndirect(_) => 0,
                Argument::RegisterIndirectDec(_) => 0,
                Argument::HighOffsetRegister(_) => 0,
                Argument::HighOffsetConstant(_) => 1,
                Argument::JumpArgument(_) => 0,
                Argument::LargeValue(_) => 2,
                Argument::SmallValue(_) => 1,
                Argument::JumpDistance(_) => 1,
                Argument::Bit(_) => 0,
                Argument::Label(_) => 2,
            })
            .sum::<u16>()
            + 1
            + cb_size
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let catagory = format!("{:?}", self.catagory);

        fn unwrap_arg(arg: &Argument) -> String {
            match arg {
                Argument::Register8Constant(reg) => format!("{:?}", reg),
                Argument::Register16Constant(reg) => format!("{:?}", reg),
                Argument::RegisterIndirectDec(reg) => format!("{:?}", reg),
                Argument::RegisterIndirect(reg) => format!("{:?}", reg),
                Argument::HighOffsetRegister(reg) => format!("{:?}", reg),
                Argument::HighOffsetConstant(val) => format!("0xFF{}", val),
                Argument::LargeValue(val) => format!("{:#X}", val),
                Argument::SmallValue(val) => format!("{:#X}", val),
                Argument::JumpDistance(val) => format!("{}", val),
                Argument::Bit(val) => format!("{}", val),
                Argument::JumpArgument(val) => format!("{:?}", val),
                Argument::Label(val) => format!("{:#X}", val),
            }
        }

        let args = self
            .args
            .iter()
            .map(|arg| format!("{}", unwrap_arg(arg)))
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{} {}", catagory, args)
    }
}

#[derive(Debug)]
enum Catagory {
    NOP,
    LD16,
    LD8,
    XOR,
    BIT,
    JR,
    INC,
    CALL,
    PUSH,
    RL,
}

#[derive(Copy, Clone, Debug)]
enum JumpCondition {
    NotZero,
}

#[derive(Copy, Clone, Debug)]
enum Argument {
    Register8Constant(RegisterLabel8),
    Register16Constant(RegisterLabel16),
    RegisterIndirectDec(RegisterLabel16),
    RegisterIndirect(RegisterLabel16),
    HighOffsetRegister(RegisterLabel8),
    HighOffsetConstant(u8),
    LargeValue(u16),
    SmallValue(u8),
    JumpDistance(i8),
    Bit(u8),
    JumpArgument(JumpCondition),
    Label(u16),
}
