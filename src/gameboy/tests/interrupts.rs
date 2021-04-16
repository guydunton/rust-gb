use super::RegisterLabel16;
use crate::Gameboy;

// !!!! Temporary test. Remove once interrupts are implemented
#[test]
fn di_does_not_do_anything() {
    let mut gb = Gameboy::new(vec![0xF3]);
    let cycles = gb.step_once();

    assert_eq!(cycles, 4);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
}
