#[allow(non_snake_case)]
pub mod Labels {
    pub const CHARACTER_RAM_START: u16 = 0x8000;
    // pub const CHARACTER_RAM_START_BLOCK_1: u16 = 0x8800; // not needed yet
    pub const CHARACTER_RAM_START_BLOCK_2: u16 = 0x9000;
    pub const BG_MAP_DATA_1_START: u16 = 0x9800;
    pub const INTERRUPT_TRIGGER: u16 = 0xFF0F;
    pub const BG_PALETTE: u16 = 0xFF47;
    pub const LCD_CONTROLS: u16 = 0xFF40;
    pub const SCROLL_Y: u16 = 0xFF42;
    pub const SCROLL_X: u16 = 0xFF43;
    pub const LCDC_Y: u16 = 0xFF44;
    pub const DMA: u16 = 0xFF46;
    pub const BOOTLOADER_DISABLE: u16 = 0xFF50;
}
