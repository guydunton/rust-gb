use super::memory_view::MemoryView;

type StoredCallback<'a> = Box<dyn FnMut(u8) + 'a>;

pub struct MemoryAdapter<'a> {
    memory: &'a mut Vec<u8>,
    callback_conditions: Vec<(u16, StoredCallback<'a>)>,
}

impl<'a> MemoryAdapter<'a> {
    pub fn new(memory: &mut Vec<u8>) -> MemoryAdapter {
        MemoryAdapter {
            memory,
            callback_conditions: vec![],
        }
    }

    pub fn add_callback<CB: 'a + FnMut(u8)>(&mut self, source: u16, callback: CB) {
        self.callback_conditions.push((source, Box::new(callback)));
    }

    pub fn set_memory_at(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;

        for (source, cb) in self.callback_conditions.iter_mut() {
            if address == *source {
                (cb)(value);
            }
        }
    }

    pub fn get_memory_at(&self, address: u16) -> u8 {
        MemoryView::new(self.memory).get_memory_at(address)
    }

    pub fn get_memory(&mut self) -> &mut Vec<u8> {
        self.memory
    }
}

#[test]
fn we_can_subscribe_to_memory_change_events() {
    let mut memory = vec![0x01, 0x02, 0x03];
    let mut add_01_changed = false;
    let mut add_02_changed = false;

    {
        let mut adapter = MemoryAdapter::new(&mut memory);
        adapter.add_callback(0x01, |_new_val| {
            add_01_changed = true;
        });
        adapter.add_callback(0x02, |_new_val| add_02_changed = true);
        adapter.set_memory_at(0x01, 0);
    }

    assert_eq!(add_01_changed, true);
    assert_eq!(add_02_changed, false);
}
