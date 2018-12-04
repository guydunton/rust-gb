
use super::endian::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RegisterLabel8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RegisterLabel16 {
    AF,
    BC,
    DE,
    HL,
    ProgramCounter,
    StackPointer,
}

#[derive(Clone)]
struct RegisterLabelIndex {
    label: RegisterLabel8,
    index: usize,
}

#[derive(Clone)]
pub struct RegisterPair {
    data_pair: [u8; 2],
    label_16_bit: RegisterLabel16,
    labels_8_bit: Vec<RegisterLabelIndex>,
}

impl RegisterPair {

    pub fn new(label_16_bit: RegisterLabel16) -> RegisterPair {
        let data_pair = [0, 0];
        let labels_8_bit = Vec::new();
        RegisterPair { data_pair, label_16_bit, labels_8_bit }
    }

    pub fn new_with_8_bit_registers(label_16_bit: RegisterLabel16, labels_8_bit: [RegisterLabel8; 2]) -> RegisterPair {

        assert_eq!(labels_8_bit.len(), 2);

        let labels_8_bit = vec![
            RegisterLabelIndex {
                label: labels_8_bit[0],
                index: 0,
            },
            RegisterLabelIndex {
                label: labels_8_bit[1],
                index: 1,
            }
        ];
        let data_pair = [0, 0];

        RegisterPair { data_pair, label_16_bit, labels_8_bit }
    }

    pub fn contains_8_bit_register(&self, label: RegisterLabel8) -> bool {
        match self.labels_8_bit.iter().find(|pair| pair.label == label) {
            Some(_) => true,
            _ => false
        }
    }

    pub fn contains_16_bit_register(&self, label: RegisterLabel16) -> bool {
        self.label_16_bit == label
    }

    pub fn perform_16_bit_read(&self) -> u16 {
        // read all the data into a u16
        be_to_u16(&self.data_pair)
    }

    pub fn perform_8_bit_read(&self, label: RegisterLabel8) -> Option<u8> {
        self.labels_8_bit.iter()
            .find(|register_label| register_label.label == label)
            .map(|register_label| self.data_pair[register_label.index])
    }

    pub fn perform_16_bit_write(&mut self, val: u16) {
        self.data_pair = u16_to_big_endian(val);
    }

    pub fn perform_8_bit_write(&mut self, label: RegisterLabel8, val: u8) -> Option<()> {
        let data_ref = &mut self.data_pair;
        self.labels_8_bit.iter_mut()
            .find(|register_label| register_label.label == label)
            .map(|register_label| data_ref[register_label.index] = val)
    }
}