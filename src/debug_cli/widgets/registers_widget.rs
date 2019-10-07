use super::super::registers::Registers;
use crate::gameboy::{RegisterLabel16, RegisterLabel8};
use std::collections::HashMap;
use super::super::layout::Print;
use crate::gameboy::Gameboy;

pub struct RegistersWidget<'a> {
    gb: &'a Gameboy,
}

impl<'a> RegistersWidget<'a> {
    pub fn new(gb: &'a Gameboy) -> RegistersWidget {
        RegistersWidget { gb }
    }

    pub fn get_registers(&self) -> Registers {
    let mut registers = HashMap::new();
    registers.insert(
        "A".to_string(),
        format!("{:#X}", self.gb.get_register_8(RegisterLabel8::A)),
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
            format!("{:#X}", self.gb.get_register_8(*register)),
        );
    });
    registers16.iter().for_each(|(label, register)| {
        registers.insert(
            label.to_string(),
            format!("{:#X}", self.gb.get_register_16(*register)),
        );
    });
    Registers { registers }
}
}

impl<'a> Print for RegistersWidget<'a> {
    fn print(&self) -> Vec<String> {
        let registers = self.get_registers();
        let register_order = vec![
            "A", "F", "AF", "B", "C", "BC", "D", "E", "DE", "H", "L", "HL", "PC", "SP",
        ];

        let mut output = Vec::new();

        output.push(String::from("----------------"));
        output.push(format!("{:<#width$} : {}", "Register", "Value", width = 8));
        output.push(String::from("----------------"));

        for register in register_order {
            output.push(format!(
                "{:<#width$} : {}",
                register,
                registers.get_register_val(&register.to_string()),
                width = 8
            ));
        }
        output
    }
}
