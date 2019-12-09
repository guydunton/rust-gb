#[allow(non_snake_case)]
pub mod Labels {
    pub const CHARACTER_RAM_START: u16 = 0x8000;
    pub const BG_MAP_DATA_1_START: u16 = 0x9800;
    pub const BG_PALETTE: u16 = 0xFF47;
    pub const V_BLANK: u16 = 0xFF40;
    pub const SCROLL_Y: u16 = 0xFF42;
    pub const SCROLL_X: u16 = 0xFF43;
    pub const LCDC_Y: u16 = 0xFF44;
}
