pub struct MemoryView<'a> {
    memory: &'a [u8],
}

impl<'a> MemoryView<'a> {
    pub fn new(memory: &[u8]) -> MemoryView {
        MemoryView { memory }
    }

    pub fn get_memory_at(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn get_memory_slice_at(&self, address: u16, size: u16) -> &'a [u8] {
        let start = address as usize;
        let end = address.saturating_add(size) as usize;
        &self.memory[start..end]
    }
}
