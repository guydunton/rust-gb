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
        let mut end = address as usize + size as usize;
        if end > self.memory.len() {
            end = self.memory.len();
        }
        &self.memory[start..end]
    }
}

#[test]
fn test_max_slice() {
    let memory = vec![0x00; 0xFFFF + 1];
    let view = MemoryView::new(&memory);

    let slice = view.get_memory_slice_at(0xFFF0, 16);
    assert_eq!(slice.len(), 16);
}
