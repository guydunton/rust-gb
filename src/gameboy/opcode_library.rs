use super::read_write_register::ReadWriteRegister;
use super::register::{ RegisterLabel8, RegisterLabel16 };
use super::endian::*;


pub fn decode_instruction(program_counter: u16, program_code: &Vec<u8>) -> OpCode {
    match program_code[program_counter as usize] {
        0x00 => OpCode::new(Catagory::NOP),
        0x31 => { 
            OpCode::new_with_args(
                Catagory::LD16, vec![
                    Argument::Register16Constant(RegisterLabel16::StackPointer),
                    Argument::LargeValue(le_to_u16(get_slice(&program_code, program_counter + 1, 2))),
                ])
        },
        0xAF => {
            OpCode::new_with_args(
                Catagory::XOR, vec![
                    Argument::Register8Constant(RegisterLabel8::A)
                ])
        },
        _ => OpCode::new(Catagory::NOP),
    }
}

pub struct OpCode {
    mneumonic: Catagory,
    arguments: Vec<Argument>,
}

impl OpCode {

    pub fn run<T: ReadWriteRegister>(&self, cpu: &mut ReadWriteRegister, memory: &mut Vec<u8>) {
        match self.mneumonic {
            Catagory::LD16 => {
                assert_eq!(self.arguments.len(), 2);

                let mut dest = |val: u16| {
                    match self.arguments[0] {
                        Argument::Register16Constant(register) => cpu.write_16_bits(register, val),
                        _ => panic!("Command does not support argument {:?}", self.arguments[0])
                    }
                };

                let source = || {
                    match self.arguments[1] {
                        Argument::LargeValue(val) => val,
                        _ => panic!("Command does not support argument {:?}", self.arguments[1])
                    }
                };

                dest(source());
            },
            Catagory::NOP => {
                // Do nothing
            },
            Catagory::XOR => {
                assert_eq!(self.arguments.len(), 1);

                match self.arguments[0] {
                    Argument::Register8Constant(register) => {
                        let new_val = cpu.read_8_bits(RegisterLabel8::A) ^ cpu.read_8_bits(register);
                        cpu.write_8_bits(RegisterLabel8::A, new_val);
                    }
                    _ => panic!("Argument not supported: {:?}", self.arguments[0])
                }

            },
        };

        // Update the program counter
        let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        cpu.write_16_bits(RegisterLabel16::ProgramCounter, program_counter + self.size());
    }

    fn new(mneumonic: Catagory) -> OpCode {
        OpCode {
            mneumonic,
            arguments: Vec::new(),
        }
    }

    fn new_with_args(mneumonic: Catagory, args: Vec<Argument>) -> OpCode {
        OpCode {
            mneumonic,
            arguments: args,
        }
    }

    fn size(&self) -> u16 {
        self.arguments.iter()
            .map(|arg| match arg {
                Argument::Register8Constant(_) => 0,
                Argument::Register16Constant(_) => 0,
                Argument::LargeValue(_) => 2,
            })
            .sum::<u16>() + 1
    }
}

fn get_slice(arr: &Vec<u8>, index: u16, size: u16) -> &[u8] {
    let start = index as usize;
    let end = (index + size) as usize;
    &arr[start..end]
}

enum Catagory {
    NOP,
    LD16,
    XOR,
}

#[derive(Copy, Clone, Debug)]
enum Argument {
    Register8Constant(RegisterLabel8),
    Register16Constant(RegisterLabel16),
    LargeValue(u16),
}
