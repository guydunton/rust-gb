use super::{Instruction, Registers};
use crate::gameboy::register::{RegisterLabel16, RegisterLabel8};
use crate::gameboy::Gameboy;
use std::collections::HashMap;
use std::u16;

pub fn print_flags(gb: &Gameboy) -> Vec<String> {
    use crate::gameboy::flags_register::*;
    let flags = vec![Flags::Z, Flags::N, Flags::H, Flags::C];

    let flag_data = flags
        .iter()
        .map(|f| (format!("{:?}", f), f))
        .map(|(label, flag)| {
            (
                label,
                match gb.get_flag(*flag) {
                    true => "1".to_string(),
                    false => "0".to_string(),
                },
            )
        })
        .map(|(label, text)| format!("{}: {}", label, text))
        .collect::<Vec<String>>();

    flag_data
}

pub fn get_registers(gb: &Gameboy) -> Registers {
    let mut registers = HashMap::new();
    registers.insert(
        "A".to_string(),
        format!("{:#X}", gb.get_register_8(RegisterLabel8::A)),
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
            format!("{:#X}", gb.get_register_8(*register)),
        );
    });
    registers16.iter().for_each(|(label, register)| {
        registers.insert(
            label.to_string(),
            format!("{:#X}", gb.get_register_16(*register)),
        );
    });
    Registers { registers }
}

/// Print the first x instructions until can't decode
pub fn print_instructions(gb: &Gameboy) -> Vec<Instruction> {
    //let mut counter = 0u16;

    let mut instructions = Vec::new();
    instructions.push(Instruction {
        address: gb.get_register_16(RegisterLabel16::ProgramCounter),
        opcode: gb.get_current_instruction(),
    });
    instructions

    // loop {
    //     let opcode = opcodes::decode_instruction(counter, &self.memory);

    //     let op_str = opcode
    //         .as_ref()
    //         .map(|op| format!("{}", op))
    //         .unwrap_or(String::from("unknown instruction"));

    //     instructions.push(Instruction {
    //         address: counter,
    //         opcode: op_str,
    //     });

    //     if opcode.is_err() {
    //         break;
    //     } else {
    //         counter += opcode.unwrap().size();
    //     }
    // }

    // instructions
}

pub fn get_pc(gb: &Gameboy) -> u16 {
    gb.get_register_16(RegisterLabel16::ProgramCounter)
}
