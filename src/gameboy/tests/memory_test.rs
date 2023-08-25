use super::Gameboy;

#[test]
fn writing_to_dma_starts_copy() {
    // LD 0xFF46 A
    let mut gb = Gameboy::new(vec![0xE0, 0x46]);
    gb.set_register_8(crate::gameboy::RegisterLabel8::A, 0x10);
    gb.set_memory_at(0x1001, 0x12);

    // DMA will copy 0x1000-0x109F to FE00-FE9F
    gb.step_once();

    assert_eq!(gb.get_memory_at(0xFE01), 0x12);
}
