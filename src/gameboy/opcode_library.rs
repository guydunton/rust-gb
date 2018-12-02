use super::read_write_register::ReadWriteRegister;
use super::register::{ RegisterLabel8, RegisterLabel16 };
use super::endian::*;



enum Mneumonic {
    NOP,
    LD,
}

#[derive(PartialEq, Debug)]
enum Argument {
    Register16Constant(RegisterLabel16),
    LargeValue(u16),
}

pub struct OpCode {
    mneumonic: Mneumonic,
    arguments: Vec<Argument>,
}

impl OpCode {
    fn new(mneumonic: Mneumonic) -> OpCode {
        OpCode {
            mneumonic,
            arguments: Vec::new(),
        }
    }

    fn new_with_args(mneumonic: Mneumonic, args: Vec<Argument>) -> OpCode {
        OpCode {
            mneumonic,
            arguments: args,
        }
    }
}

pub fn decode_instruction(program_counter: u16, program_code: &Vec<u8>) -> OpCode {
    match program_code[program_counter as usize] {
        0x00 => OpCode::new(Mneumonic::NOP),
        0x31 => { 
            let start = (program_counter + 1) as usize;
            let end = (program_counter + 3) as usize;
            let slice = &program_code[start..end];
            OpCode::new_with_args(
            Mneumonic::LD, vec![
                Argument::Register16Constant(RegisterLabel16::StackPointer),
                Argument::LargeValue(le_to_u16(slice)),
            ]
        )},
        _ => OpCode::new(Mneumonic::NOP),
    }
}

fn expect(arg: Argument, expected: Argument) -> Argument {
    if arg == expected {
        return arg;
    }
    else {
        panic!("Unexpected value occured. Expected: {:?}, found: {:?}", expected, arg);
    }
}

impl OpCode {

    

    pub fn run<T: ReadWriteRegister>(&self, cpu: &mut ReadWriteRegister, memory: &mut Vec<u8>) {
        match self.mneumonic {
            Mneumonic::LD => {
                assert_eq!(self.arguments.len(), 2);
                match self.arguments[0] {
                    Argument::Register16Constant(register) => {
                        cpu.write_16_bits(register, expect(self.arguments[1], Argument::LargeValue));
                    },
                    _ => panic!("Can't load the contents of a register into a value")
                }
            },
            Mneumonic::NOP => {
                // Do nothing
            },
            _ => {
                panic!("Created an opcode that we can't run")
            }
        };

        // Update the program counter
        let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        cpu.write_16_bits(RegisterLabel16::ProgramCounter, program_counter + self.size());
    }

    fn size(&self) -> u16 {
        self.arguments.iter()
            .map(|arg| match arg {
                Argument::Register16Constant(_) => 0,
                Argument::LargeValue(_) => 2,
            })
            .sum::<u16>() + 1
    }
}

