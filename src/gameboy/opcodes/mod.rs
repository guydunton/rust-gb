mod argument;
mod bit;
mod call;
mod inc;
mod jmp;
mod ld16;
mod ld8;
mod push;
mod rotate_left;
mod rotate_left_a;
mod xor;

use self::argument::{size_in_bytes, Argument, JumpCondition};
use super::flags_register::*;
use super::read_write_register::ReadWriteRegister;
use super::register::{RegisterLabel16, RegisterLabel8};
use std::fmt;

fn arg_from_str(arg: &str, index: u16, memory: &[u8]) -> Result<Argument, String> {
    let result = match arg {
        "DE" => Argument::Register16Constant(RegisterLabel16::DE),
        "HL" => Argument::Register16Constant(RegisterLabel16::HL),
        "SP" => Argument::Register16Constant(RegisterLabel16::StackPointer),
        "BC" => Argument::Register16Constant(RegisterLabel16::BC),
        "(HL-)" => Argument::RegisterIndirectDec(RegisterLabel16::HL),
        "A" => Argument::Register8Constant(RegisterLabel8::A),
        "B" => Argument::Register8Constant(RegisterLabel8::B),
        "C" => Argument::Register8Constant(RegisterLabel8::C),
        "H" => Argument::Register8Constant(RegisterLabel8::H),
        "(C)" => Argument::HighOffsetRegister(RegisterLabel8::C),
        "(DE)" => Argument::RegisterIndirect(RegisterLabel16::DE),
        "(HL)" => Argument::RegisterIndirect(RegisterLabel16::HL),
        "(a8)" => Argument::HighOffsetConstant(memory[index as usize + 1]),
        "a16" => Argument::Label(u16::from_le_bytes([
            memory[(index + 1) as usize],
            memory[(index + 2) as usize],
        ])),
        "d16" => Argument::LargeValue(u16::from_le_bytes([
            memory[(index + 1) as usize],
            memory[(index + 2) as usize],
        ])),
        "d8" => Argument::SmallValue(memory[(index + 1) as usize]),
        "NZ" => Argument::JumpArgument(JumpCondition::NotZero),
        "r8" => Argument::JumpDistance(memory[(index + 1) as usize] as i8),
        "7" => Argument::Bit(7),
        _ => return Err(format!("Unknown argument: {}", arg)),
    };
    Ok(result)
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
        "PUSH" => Catagory::PUSH,
        "RL" => Catagory::RL,
        _ => Catagory::NOP,
    }
}

pub fn decode_instruction(program_counter: u16, program_code: &[u8]) -> Result<OpCode, String> {
    let code = program_code[program_counter as usize];

    let opcode = |text: &str| -> Result<OpCode, String> {
        let parts = text.split(' ').collect::<Vec<&str>>();
        let catagory = catagory_from_str(parts[0]);

        let args = parts[1..]
            .iter()
            .map(|arg| arg_from_str(arg, program_counter, program_code));

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
        0x17 => Ok(OpCode::new(Catagory::RLA, vec![])),
        0x1A => opcode("LD8 A (DE)"),
        0x20 => opcode("JR NZ r8"),
        0x21 => opcode("LD16 HL d16"),
        0x31 => opcode("LD16 SP d16"),
        0x32 => opcode("LD8 (HL-) A"),
        0x3E => opcode("LD8 A d8"),
        0x4F => opcode("LD8 C A"),
        0x77 => opcode("LD8 (HL) A"),
        0xAF => opcode("XOR A"),
        0xC5 => opcode("PUSH BC"),
        0xCB => {
            // 0xCB is prefix and the next byte shows the actual instruction
            let cb_instruction = program_code[program_counter as usize + 1];
            match cb_instruction {
                0x11 => opcode("RL C"),
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
            Catagory::RLA => {
                cycles += self.run_rla::<T>(cpu, memory);
            }
        };

        cycles
    }

    fn new(catagory: Catagory, args: Vec<Argument>) -> OpCode {
        OpCode { catagory, args }
    }

    pub fn size(&self) -> u16 {
        let type_size = catagory_size(self.catagory);
        self.args.iter().map(|arg| size_in_bytes(*arg)).sum::<u16>() + type_size
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let catagory = format!("{:?}", self.catagory);

        let args = self
            .args
            .iter()
            .map(|arg| format!("{}", arg))
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{} {}", catagory, args)
    }
}

#[derive(Debug, Copy, Clone)]
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
    RLA,
}

fn is_cb_catagory(catagory: Catagory) -> bool {
    match catagory {
        Catagory::RL => true,
        Catagory::BIT => true,
        _ => false,
    }
}

fn catagory_size(catagory: Catagory) -> u16 {
    let cb_size = if is_cb_catagory(catagory) { 1 } else { 0 };
    let total_size = cb_size + 1;
    total_size
}
