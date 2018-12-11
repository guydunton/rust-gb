use super::endian::*;
use super::flags_register::*;
use super::read_write_register::ReadWriteRegister;
use super::register::{RegisterLabel16, RegisterLabel8};

pub fn decode_instruction(program_counter: u16, program_code: &[u8]) -> OpCode {
    let code = program_code[program_counter as usize];
    match code {
        0x00 => OpCode::new(Catagory::NOP),
        0x21 => OpCode::new_with_args(
            Catagory::LD16,
            vec![
                Argument::Register16Constant(RegisterLabel16::HL),
                Argument::LargeValue(le_to_u16(get_slice(&program_code, program_counter + 1, 2))),
            ],
        ),
        0x31 => OpCode::new_with_args(
            Catagory::LD16,
            vec![
                Argument::Register16Constant(RegisterLabel16::StackPointer),
                Argument::LargeValue(le_to_u16(get_slice(&program_code, program_counter + 1, 2))),
            ],
        ),
        0x32 => OpCode::new_with_args(
            Catagory::LD8,
            vec![
                Argument::RegisterIndirectDec(RegisterLabel16::HL),
                Argument::Register8Constant(RegisterLabel8::A),
            ],
        ),
        0xAF => OpCode::new_with_args(
            Catagory::XOR,
            vec![Argument::Register8Constant(RegisterLabel8::A)],
        ),
        0xCB => {
            // 0xCB is prefix and the next byte shows the actual instruction
            let cb_instruction = program_code[program_counter as usize + 1];
            match cb_instruction {
                0x7C => OpCode::new_with_args(
                    Catagory::BIT,
                    vec![
                        Argument::Bit(7),
                        Argument::Register8Constant(RegisterLabel8::H),
                    ], // TODO: Need a declarative way of specifying flags settings
                ),
                _ => panic!("Unknown command 0xCB {:#X}", cb_instruction),
            }
        }
        _ => panic!("Unkown command {:#X}", code),
    }
}

pub struct OpCode {
    catagory: Catagory,
    args: Vec<Argument>,
}

impl OpCode {
    pub fn run<T: ReadWriteRegister>(&self, cpu: &mut dyn ReadWriteRegister, memory: &mut Vec<u8>) {
        match self.catagory {
            Catagory::LD16 => {
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
            }
            Catagory::LD8 => {
                assert_eq!(self.args.len(), 2);
                {
                    let mut dest = |val: u8| match self.args[0] {
                        Argument::RegisterIndirectDec(register) => {
                            memory[cpu.read_16_bits(register) as usize] = val
                        }
                        _ => panic!("Command does not support argument {:?}", self.args[0]),
                    };

                    let source = || match self.args[1] {
                        Argument::Register8Constant(register) => cpu.read_8_bits(register),
                        _ => panic!("Command does not support argument {:?}", self.args[0]),
                    };

                    dest(source());
                }

                match self.args[0] {
                    Argument::RegisterIndirectDec(register) => {
                        let new_val = cpu.read_16_bits(register) - 1;
                        cpu.write_16_bits(register, new_val);
                    }
                    _ => {} // Do nothing
                }
            }
            Catagory::NOP => {
                // Do nothing
            }
            Catagory::XOR => {
                assert_eq!(self.args.len(), 1);

                match self.args[0] {
                    Argument::Register8Constant(register) => {
                        let new_val =
                            cpu.read_8_bits(RegisterLabel8::A) ^ cpu.read_8_bits(register);
                        cpu.write_8_bits(RegisterLabel8::A, new_val);
                        cpu.write_8_bits(RegisterLabel8::F, 0);
                    }
                    _ => panic!("Argument not supported: {:?}", self.args[0]),
                }
            }
            Catagory::BIT => {
                assert_eq!(self.args.len(), 2);

                match (self.args[0], self.args[1]) {
                    (Argument::Bit(bit), Argument::Register8Constant(register)) => {
                        let register = cpu.read_8_bits(register);
                        let mut flags = cpu.read_8_bits(RegisterLabel8::F);

                        // TODO: Can we create a version which uses a cpu?
                        let result = (((0x1 << bit) ^ register) >> bit) == 1;
                        flags = set_flag(flags, Flags::Z, result);
                        flags = set_flag(flags, Flags::N, false);
                        flags = set_flag(flags, Flags::H, true);
                        cpu.write_8_bits(RegisterLabel8::F, flags);
                    }
                    _ => panic!("Invalid arguments"),
                }
            }
        };

        // Update the program counter
        let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        cpu.write_16_bits(
            RegisterLabel16::ProgramCounter,
            program_counter + self.size(),
        );
    }

    fn new(catagory: Catagory) -> OpCode {
        OpCode {
            catagory,
            args: Vec::new(),
        }
    }

    fn new_with_args(catagory: Catagory, args: Vec<Argument>) -> OpCode {
        OpCode { catagory, args }
    }

    fn size(&self) -> u16 {
        self.args
            .iter()
            .map(|arg| match arg {
                Argument::Register8Constant(_) => 0,
                Argument::Register16Constant(_) => 0,
                Argument::RegisterIndirectDec(_) => 0,
                Argument::LargeValue(_) => 2,
                Argument::Bit(_) => 1,
            })
            .sum::<u16>()
            + 1
    }
}

fn get_slice(arr: &[u8], index: u16, size: u16) -> &[u8] {
    let start = index as usize;
    let end = (index + size) as usize;
    &arr[start..end]
}

enum Catagory {
    NOP,
    LD16,
    LD8,
    XOR,
    BIT,
}

#[derive(Copy, Clone, Debug)]
enum Argument {
    Register8Constant(RegisterLabel8),
    Register16Constant(RegisterLabel16),
    RegisterIndirectDec(RegisterLabel16),
    LargeValue(u16),
    Bit(u8),
}
