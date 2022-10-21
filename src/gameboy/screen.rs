#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenColor {
    White = 0,
    Light = 1,
    Dark = 2,
    Black = 3,
}
