use super::cpu::CPU;
use super::debug::{Instruction, Registers};
use super::opcodes;
use super::read_write_register::ReadWriteRegister;
use super::register::{RegisterLabel16, RegisterLabel8};
use super::Gameboy;
use std::collections::HashMap;
use std::u16;

impl Gameboy {
    pub fn print_flags(&self) -> Vec<String> {
        use super::flags_register::*;
        let flags = vec![Flags::Z, Flags::N, Flags::H, Flags::C];

        let flag_data = flags
            .iter()
            .map(|f| (format!("{:?}", f), f))
            .map(|(label, flag)| {
                (
                    label,
                    match read_flag::<CPU>(&self.cpu, *flag) {
                        true => "1".to_string(),
                        false => "0".to_string(),
                    },
                )
            })
            .map(|(label, text)| format!("{}: {}", label, text))
            .collect::<Vec<String>>();

        flag_data
    }

    pub fn get_registers(&self) -> Registers {
        let mut registers = HashMap::new();
        registers.insert(
            "A".to_string(),
            format!("{:#X}", self.cpu.read_8_bits(RegisterLabel8::A)),
        );
        let registers8 = vec![
            ("A", RegisterLabel8::A),
            ("F", RegisterLabel8::F),
            ("B", RegisterLabel8::B),
            ("C", RegisterLabel8::C),
            ("D", RegisterLabel8::D),
            ("E", RegisterLabel8::E),
            ("H", RegisterLabel8::H),
            ("L", RegisterLabel8::L),
        ];
        let registers16 = vec![
            ("AF", RegisterLabel16::AF),
            ("BC", RegisterLabel16::BC),
            ("DE", RegisterLabel16::DE),
            ("HL", RegisterLabel16::HL),
            ("SP", RegisterLabel16::StackPointer),
            ("PC", RegisterLabel16::ProgramCounter),
        ];

        registers8.iter().for_each(|(label, register)| {
            registers.insert(
                label.to_string(),
                format!("{:#X}", self.cpu.read_8_bits(*register)),
            );
        });
        registers16.iter().for_each(|(label, register)| {
            registers.insert(
                label.to_string(),
                format!("{:#X}", self.cpu.read_16_bits(*register)),
            );
        });
        Registers { registers }
    }

    /// Print the first x instructions until can't decode
    pub fn print_instructions(&self) -> Vec<Instruction> {
        let mut counter = 0u16;

        let mut instructions = Vec::new();

        loop {
            let opcode = opcodes::decode_instruction(counter, &self.memory);

            let op_str = opcode
                .as_ref()
                .map(|op| format!("{}", op))
                .unwrap_or(String::from("unknown instruction"));

            instructions.push(Instruction {
                address: counter,
                opcode: op_str,
            });

            if opcode.is_err() {
                break;
            } else {
                counter += opcode.unwrap().size();
            }
        }

        instructions
    }

    pub fn get_pc(&self) -> u16 {
        self.cpu.read_16_bits(RegisterLabel16::ProgramCounter)
    }
}
