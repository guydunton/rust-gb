use std::collections::HashMap;

pub struct Registers {
    pub registers: HashMap<String, String>,
}

impl Registers {
    pub fn get_register_val(&self, register: &String) -> String {
        self.registers
            .get(register)
            .map(|x| x.clone())
            .unwrap_or("Invalid register".to_string())
    }
}
