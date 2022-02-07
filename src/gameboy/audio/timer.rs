#[derive(Debug, PartialEq)]
pub enum TickResult {
    Ticked,
    Noop,
    Disabled,
}
pub struct Timer {
    count: i32,
    length: i32,
    enabled: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            count: 0,
            length: 0,
            enabled: false,
        }
    }

    pub fn start(&mut self, length: i32) {
        self.count = length;
        self.length = length;
        self.enabled = true;
    }

    pub fn tick(&mut self, dt: u32) -> TickResult {
        if self.enabled {
            self.count -= dt as i32;

            if self.count <= 0 {
                self.count += self.length;
                return TickResult::Ticked;
            } else {
                return TickResult::Noop;
            }
        }
        TickResult::Disabled
    }
}
