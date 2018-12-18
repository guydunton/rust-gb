use super::endian::*;
use super::flags_register::*;
use super::read_write_register::ReadWriteRegister;
use super::register::{RegisterLabel16, RegisterLabel8};

fn catagory_from_str(cat: &str) -> Catagory {
    match cat {
        "NOP" => Catagory::NOP,
        "LD16" => Catagory::LD16,
        "LD8" => Catagory::LD8,
        "XOR" => Catagory::XOR,
        "BIT" => Catagory::BIT,
        _ => Catagory::NOP,
    }
}

pub fn decode_instruction(program_counter: u16, program_code: &[u8]) -> OpCode {
    let code = program_code[program_counter as usize];

    // Needs to be closure to capture values from memory
    let arg_from_str = |arg: &str| -> Argument {
        match arg {
            "HL" => Argument::Register16Constant(RegisterLabel16::HL),
            "SP" => Argument::Register16Constant(RegisterLabel16::StackPointer),
            "(HL-)" => Argument::RegisterIndirectDec(RegisterLabel16::HL),
            "A" => Argument::Register8Constant(RegisterLabel8::A),
            "H" => Argument::Register8Constant(RegisterLabel8::H),
            "d16" => {
                Argument::LargeValue(le_to_u16(get_slice(&program_code, program_counter + 1, 2)))
            }
            "7" => Argument::Bit(7),
            _ => panic!("Unknown argument: {}", arg),
        }
    };

    let opcode = |text: &str| -> OpCode {
        let parts = text.split(' ').collect::<Vec<&str>>();
        let catagory = catagory_from_str(parts[0]);
        let args = parts[1..]
            .iter()
            .map(|arg| arg_from_str(arg))
            .collect::<Vec<Argument>>();

        OpCode::new(catagory, args)
    };

    match code {
        0x00 => opcode("NOP"),
        0x20 => OpCode::new(
            Catagory::JR,
            vec![
                Argument::JumpArgument(JumpCondition::NotZero),
                Argument::JumpDistance(program_code[(program_counter + 1) as usize] as i8),
            ],
        ),
        0x21 => opcode("LD16 HL d16"),
        0x31 => opcode("LD16 SP d16"),
        0x32 => opcode("LD8 (HL-) A"),
        0xAF => opcode("XOR A"),
        0xCB => {
            // 0xCB is prefix and the next byte shows the actual instruction
            let cb_instruction = program_code[program_counter as usize + 1];
            match cb_instruction {
                0x7C => opcode("BIT 7 H"),
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
    pub fn run<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        memory: &mut Vec<u8>,
    ) -> u32 {
        // Update the program counter if not a jump
        let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        cpu.write_16_bits(
            RegisterLabel16::ProgramCounter,
            program_counter + self.size(),
        );

        let mut cycles = 0;

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

                cycles += 12;
            }
            Catagory::LD8 => {
                assert_eq!(self.args.len(), 2);
                {
                    let mut dest = |val: u8| match self.args[0] {
                        Argument::RegisterIndirectDec(register) => {
                            cycles += 4;
                            memory[cpu.read_16_bits(register) as usize] = val
                        }
                        _ => panic!("Command does not support argument {:?}", self.args[0]),
                    };

                    let source = || match self.args[1] {
                        Argument::Register8Constant(register) => cpu.read_8_bits(register),
                        _ => panic!("Command does not support argument {:?}", self.args[0]),
                    };

                    dest(source());

                    cycles += 4;
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
                cycles += 4;
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

                cycles += 4;
            }
            Catagory::BIT => {
                assert_eq!(self.args.len(), 2);

                match (self.args[0], self.args[1]) {
                    (Argument::Bit(bit), Argument::Register8Constant(register)) => {
                        let register = cpu.read_8_bits(register);

                        let result = (((0x1 << bit) ^ register) >> bit) == 1;
                        write_flag::<T>(cpu, Flags::Z, result);
                        write_flag::<T>(cpu, Flags::N, false);
                        write_flag::<T>(cpu, Flags::H, true);
                    }
                    _ => panic!("Invalid arguments"),
                }

                cycles += 12;
            }
            Catagory::JR => {
                assert_eq!(self.args.len(), 2);

                // 8 cycles by default
                cycles += 8;

                // Arg 1 is the condition
                let condition = match self.args[0] {
                    Argument::JumpArgument(condition) => condition,
                    _ => panic!("Invalid argument for jump statement {:?}", self.args[0]),
                };

                let condition_checker = || -> bool {
                    match condition {
                        JumpCondition::NotZero => read_flag::<T>(cpu, Flags::Z) == false,
                    }
                };

                if condition_checker() {
                    // Arg 2 is relative location

                    let distance = match self.args[1] {
                        Argument::JumpDistance(distance) => distance,
                        _ => panic!("Invalid argument"),
                    };

                    let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
                    cpu.write_16_bits(
                        RegisterLabel16::ProgramCounter,
                        (i32::from(program_counter) + i32::from(distance)) as u16,
                    );

                    cycles += 4;
                }
            }
        };

        cycles
    }

    pub fn to_string(&self) -> String {
        let catagory = format!("{:?}", self.catagory);

        let args = self
            .args
            .iter()
            .map(|arg| format!("{:?}", arg))
            .collect::<Vec<String>>()
            .join(" ");
        format!("{} {}", catagory, args)
    }

    fn new(catagory: Catagory, args: Vec<Argument>) -> OpCode {
        OpCode { catagory, args }
    }

    fn size(&self) -> u16 {
        self.args
            .iter()
            .map(|arg| match arg {
                Argument::Register8Constant(_) => 0,
                Argument::Register16Constant(_) => 0,
                Argument::RegisterIndirectDec(_) => 0,
                Argument::JumpArgument(_) => 0,
                Argument::LargeValue(_) => 2,
                Argument::JumpDistance(_) => 1,
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

#[derive(Debug)]
enum Catagory {
    NOP,
    LD16,
    LD8,
    XOR,
    BIT,
    JR,
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
    LargeValue(u16),
    JumpDistance(i8),
    Bit(u8),
    JumpArgument(JumpCondition),
}
