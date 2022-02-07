use std::collections::HashMap;

pub struct Registers {
    pub registers: HashMap<String, String>,
}

impl Registers {
    pub fn get_register_val(&self, register: &str) -> String {
        self.registers
            .get(register)
            .cloned()
            .unwrap_or_else(|| "Invalid register".to_string())
    }
}
