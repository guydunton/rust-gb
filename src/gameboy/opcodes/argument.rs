use super::{RegisterLabel16, RegisterLabel8};
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Argument {
    Register8Constant(RegisterLabel8),
    Register16Constant(RegisterLabel16),
    RegisterIndirectDec(RegisterLabel16),
    RegisterIndirect(RegisterLabel16),
    HighOffsetRegister(RegisterLabel8),
    HighOffsetConstant(u8),
    LargeValue(u16),
    SmallValue(u8),
    JumpDistance(i8),
    Bit(u8),
    JumpArgument(JumpCondition),
    Label(u16),
}

pub fn size_in_bytes(argument: Argument) -> u16 {
    match argument {
        Argument::Register8Constant(_) => 0,
        Argument::Register16Constant(_) => 0,
        Argument::RegisterIndirect(_) => 0,
        Argument::RegisterIndirectDec(_) => 0,
        Argument::HighOffsetRegister(_) => 0,
        Argument::HighOffsetConstant(_) => 1,
        Argument::JumpArgument(_) => 0,
        Argument::LargeValue(_) => 2,
        Argument::SmallValue(_) => 1,
        Argument::JumpDistance(_) => 1,
        Argument::Bit(_) => 0,
        Argument::Label(_) => 2,
    }
}

#[derive(Copy, Clone, Debug)]
pub enum JumpCondition {
    NotZero,
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Argument::Register8Constant(reg) => write!(f, "{:?}", reg),
            Argument::Register16Constant(reg) => write!(f, "{:?}", reg),
            Argument::RegisterIndirectDec(reg) => write!(f, "{:?}", reg),
            Argument::RegisterIndirect(reg) => write!(f, "{:?}", reg),
            Argument::HighOffsetRegister(reg) => write!(f, "{:?}", reg),
            Argument::HighOffsetConstant(val) => write!(f, "0xFF{}", val),
            Argument::LargeValue(val) => write!(f, "{:#X}", val),
            Argument::SmallValue(val) => write!(f, "{:#X}", val),
            Argument::JumpDistance(val) => write!(f, "{}", val),
            Argument::Bit(val) => write!(f, "{}", val),
            Argument::JumpArgument(val) => write!(f, "{:?}", val),
            Argument::Label(val) => write!(f, "{:#X}", val),
        }
    }
}