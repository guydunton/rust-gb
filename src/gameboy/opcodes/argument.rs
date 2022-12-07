use super::super::{RegisterLabel16, RegisterLabel8};
use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Argument {
    Register8Constant(RegisterLabel8),
    Register16Constant(RegisterLabel16),
    RegisterIndirectDec(RegisterLabel16),
    RegisterIndirectInc(RegisterLabel16),
    RegisterIndirect(RegisterLabel16),
    HighOffsetRegister(RegisterLabel8),
    HighOffsetConstant(u8),
    LargeValue(u16),
    SmallValue(u8),
    JumpDistance(i8),
    Bit(u8),
    JumpCondition(JumpCondition),
    Label(u16),
    AddressIndirect(u16),
    SPOffset(i8),
    Vector(u16),
    None,
}

pub fn size_in_bytes(argument: Argument) -> u16 {
    match argument {
        Argument::Register8Constant(_) => 0,
        Argument::Register16Constant(_) => 0,
        Argument::RegisterIndirect(_) => 0,
        Argument::RegisterIndirectDec(_) => 0,
        Argument::RegisterIndirectInc(_) => 0,
        Argument::HighOffsetRegister(_) => 0,
        Argument::HighOffsetConstant(_) => 1,
        Argument::JumpCondition(_) => 0,
        Argument::LargeValue(_) => 2,
        Argument::SmallValue(_) => 1,
        Argument::JumpDistance(_) => 1,
        Argument::Bit(_) => 0,
        Argument::Label(_) => 2,
        Argument::AddressIndirect(_) => 2,
        Argument::SPOffset(_) => 1,
        Argument::Vector(_) => 0,
        Argument::None => 0,
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum JumpCondition {
    NotZero,
    Zero,
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Argument::Register8Constant(reg) => write!(f, "{:?}", reg),
            Argument::Register16Constant(reg) => write!(f, "{:?}", reg),
            Argument::RegisterIndirectDec(reg) => write!(f, "({:?})-", reg),
            Argument::RegisterIndirectInc(reg) => write!(f, "({:?})+", reg),
            Argument::RegisterIndirect(reg) => write!(f, "({:?})", reg),
            Argument::HighOffsetRegister(reg) => write!(f, "(0xFF+{:?})", reg),
            Argument::HighOffsetConstant(val) => write!(f, "(0xFF{})", val),
            Argument::LargeValue(val) => write!(f, "{:#X}", val),
            Argument::SmallValue(val) => write!(f, "{:#X}", val),
            Argument::JumpDistance(val) => write!(f, "{}", val),
            Argument::Bit(val) => write!(f, "{}", val),
            Argument::JumpCondition(val) => write!(f, "{:?}", val),
            Argument::Label(val) => write!(f, "{:#X}", val),
            Argument::AddressIndirect(val) => write!(f, "({:#X})", val),
            Argument::SPOffset(val) => write!(f, "SP+{:#X}", val),
            Argument::Vector(address) => write!(f, "{:#X}H", address),
            Argument::None => write!(f, ""),
        }
    }
}

pub fn arg_from_str(arg: &str, index: u16, memory: &[u8]) -> Result<Argument, &'static str> {
    let result = match arg {
        "BC" => Argument::Register16Constant(RegisterLabel16::BC),
        "DE" => Argument::Register16Constant(RegisterLabel16::DE),
        "HL" => Argument::Register16Constant(RegisterLabel16::HL),
        "AF" => Argument::Register16Constant(RegisterLabel16::AF),
        "SP" => Argument::Register16Constant(RegisterLabel16::StackPointer),
        "(HL-)" => Argument::RegisterIndirectDec(RegisterLabel16::HL),
        "(HL+)" => Argument::RegisterIndirectInc(RegisterLabel16::HL),
        "A" => Argument::Register8Constant(RegisterLabel8::A),
        "B" => Argument::Register8Constant(RegisterLabel8::B),
        "C" => Argument::Register8Constant(RegisterLabel8::C),
        "D" => Argument::Register8Constant(RegisterLabel8::D),
        "E" => Argument::Register8Constant(RegisterLabel8::E),
        "H" => Argument::Register8Constant(RegisterLabel8::H),
        "L" => Argument::Register8Constant(RegisterLabel8::L),
        "00H" => Argument::Vector(0x0000),
        "10H" => Argument::Vector(0x0010),
        "20H" => Argument::Vector(0x0020),
        "30H" => Argument::Vector(0x0030),
        "08H" => Argument::Vector(0x0008),
        "18H" => Argument::Vector(0x0018),
        "28H" => Argument::Vector(0x0028),
        "38H" => Argument::Vector(0x0038),
        "(C)" => Argument::HighOffsetRegister(RegisterLabel8::C),
        "(BC)" => Argument::RegisterIndirect(RegisterLabel16::BC),
        "(DE)" => Argument::RegisterIndirect(RegisterLabel16::DE),
        "(HL)" => Argument::RegisterIndirect(RegisterLabel16::HL),
        "(a8)" => Argument::HighOffsetConstant(memory[index as usize + 1]),
        "(a16)" => Argument::AddressIndirect(u16::from_le_bytes([
            memory[index as usize + 1],
            memory[index as usize + 2],
        ])),
        "a16" => Argument::Label(u16::from_le_bytes([
            memory[(index + 1) as usize],
            memory[(index + 2) as usize],
        ])),
        "d16" => Argument::LargeValue(u16::from_le_bytes([
            memory[(index + 1) as usize],
            memory[(index + 2) as usize],
        ])),
        "d8" => Argument::SmallValue(memory[(index + 1) as usize]),
        "NZ" => Argument::JumpCondition(JumpCondition::NotZero),
        "Z" => Argument::JumpCondition(JumpCondition::Zero),
        "r8" => Argument::JumpDistance(memory[(index + 1) as usize] as i8),
        "SP+r8" => Argument::SPOffset(memory[(index + 1) as usize] as i8),
        "7" => Argument::Bit(7),
        _ => return Err("Unknown argument"),
    };
    Ok(result)
}
