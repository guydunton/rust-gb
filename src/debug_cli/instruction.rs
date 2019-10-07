pub struct Instruction {
    pub address: u16,
    pub opcode: String,
}

impl Instruction {
    pub fn get_address(&self) -> u16 {
        self.address
    }

    pub fn get_opcode(&self) -> String {
        self.opcode.clone()
    }
}
