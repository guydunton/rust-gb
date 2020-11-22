use crate::gameboy::Gameboy;

#[allow(dead_code)]
pub fn infinite_loop_gb<'a, F>(callback: F) -> Gameboy<'a>
where
    F: FnMut(i16) + 'a,
{
    // Each loop will be 16 clocks & take 2 steps
    // NOP
    // JR -3
    let gb = Gameboy::new_with_audio(vec![0x00, 0x18, 0xFD], callback);
    gb
}

#[test]
fn can_construct_gb_with_alu() {
    let mut audio_data: Vec<i16> = Vec::new();

    {
        // Put everything in scope to allow us to query audio_data
        let callback = |val| {
            audio_data.push(val);
        };

        let mut gb = infinite_loop_gb(callback);

        let breakpoints = vec![];
        gb.tick(1.0 / 60.0, &breakpoints);
    }

    assert_ne!(audio_data.len(), 0);
}
