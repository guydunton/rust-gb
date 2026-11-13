use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct OpcodeWriter {
    opcode_record: HashMap<u16, String>,
    filename: PathBuf,
}

impl OpcodeWriter {
    pub fn new(filename: &Path) -> Self {
        let mut opcode_record: HashMap<u16, String> = HashMap::new();
        opcode_record.reserve(4000);
        Self {
            opcode_record,
            filename: filename.to_path_buf(),
        }
    }

    pub fn store_opcode(&mut self, address: u16, opcode: String) {
        let _ = self.opcode_record.insert(address, opcode);
    }

    pub fn write_file(&mut self) {
        let mut file = File::create(&self.filename).unwrap();
        let mut keys: Vec<u16> = self.opcode_record.keys().copied().collect();
        keys.sort();

        for key in keys {
            let opcode = self.opcode_record.entry(key).or_default();
            writeln!(file, "{:#06x}: {}", key, opcode).unwrap();
        }
    }
}
