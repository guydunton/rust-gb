use super::register::{ RegisterLabel8, RegisterLabel16 };

pub trait ReadWriteRegister {
    fn write_16_bits(&mut self, label: RegisterLabel16, value: u16);
    fn write_8_bits(&mut self, label: RegisterLabel8, value: u8);
    fn read_16_bits(& self, label: RegisterLabel16) -> u16;
    fn read_8_bits(& self, label: RegisterLabel8) -> u8;
}