#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum ScreenColor {
    White = 0,
    Light = 1,
    Dark = 2,
    Black = 3,
}

const MAX_WIDTH: usize = 160;
const MAX_HEIGHT: usize = 144;
const MAX_LENGTH: usize = MAX_WIDTH * MAX_HEIGHT;

pub struct Screen {
    pixels: Vec<ScreenColor>,
    index: usize,
}

impl Screen {
    pub fn new() -> Screen {
        let pixels = vec![ScreenColor::White; MAX_LENGTH];
        let index = 0;

        Screen { pixels, index }
    }

    pub fn push_pixels(&mut self, pixels: &[ScreenColor]) {
        for p in pixels.iter() {
            self.pixels[self.index] = *p;
            self.index = (self.index + 1) % MAX_LENGTH;
        }
    }

    pub fn pixels(&self) -> &Vec<ScreenColor> {
        &self.pixels
    }
}
