pub fn cb_code_to_opcode(code: u8) -> Result<&'static str, String> {
    match code {
        0x11 => Ok("RL C"),
        0x7C => Ok("BIT 7 H"),
        0xAA => Ok("Thing"),
        _ => Err(format!("Unknown command 0xCB {:#X}", code)),
    }
}
