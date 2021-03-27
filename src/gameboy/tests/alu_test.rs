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

        gb.tick(1.0 / 60.0);
    }

    assert_ne!(audio_data.len(), 0);
}

#[test]
fn trigger_bit_is_reset_after_frame() {
    let mut gb = infinite_loop_gb(|_| {});

    // Enable the trigger bit
    gb.set_memory_at(0xFF14, 0b1000_0000);

    gb.tick(1.0 / 60.0);

    // The trigger bit will be disabled again
    assert_eq!(gb.get_memory_at(0xFF14) & 0b1000_0000, 0b0000_0000);
}

#[test]
fn no_sound_if_volume_0() {
    let mut audio_data: Vec<i16> = Vec::new();
    {
        let mut gb = infinite_loop_gb(|val| {
            audio_data.push(val);
        });

        // enable sound
        gb.set_memory_at(0xFF14, 0b1000_0000);
        gb.tick(1.0 / 60.0);
    }

    assert_eq!(audio_data.len() > 0, true);
    assert_eq!(audio_data.iter().all(|&val| val == 0), true);
}

#[test]
fn setting_volume_enables_output() {
    let mut audio_data: Vec<i16> = Vec::new();
    {
        let mut gb = infinite_loop_gb(|val| {
            audio_data.push(val);
        });

        // Set the volume to max
        gb.set_memory_at(0xFF12, 0b1111_0000);

        // enable sound 1
        gb.set_memory_at(0xFF14, 0b1000_0000);
        gb.tick(1.0 / 60.0);
    }

    assert_eq!(audio_data.len() > 0, true);
    assert_eq!(audio_data.iter().any(|&val| val != 0), true);
}

#[allow(dead_code)]
fn run_gb_with_settings(vol: u8, freq: u16, duty: u8, period: u8) -> Vec<i16> {
    let mut audio_data: Vec<i16> = Vec::new();
    {
        let mut gb = infinite_loop_gb(|val| {
            audio_data.push(val);
        });

        // Set the duty
        gb.set_memory_at(0xFF11, gb.get_memory_at(0xFF11) | (duty << 6));

        // Set the volume & period
        gb.set_memory_at(0xFF12, (vol << 4) | period);

        // Set the frequency
        let [freq_msb, freq_lsb] = freq.to_be_bytes();
        gb.set_memory_at(0xFF13, freq_lsb);
        gb.set_memory_at(0xFF14, gb.get_memory_at(0xFF14) | (freq_msb & 0b0111));

        // enable sound 1
        gb.set_memory_at(0xFF14, gb.get_memory_at(0xFF14) | 0b1000_0000);
        gb.tick(1.0 / 60.0);
    }

    audio_data
}

#[test]
fn increase_volume_increases_output() {
    let sound1 = run_gb_with_settings(0b1111, u8::MAX as u16, 0, 0);
    let sound2 = run_gb_with_settings(0b0001, u8::MAX as u16, 0, 0);

    let total1: i16 = sound1.iter().sum();
    let total2: i16 = sound2.iter().sum();

    println!("total 1: {}", &total1);
    println!("total 2: {}", &total2);

    assert!(total1 > total2);
}

#[test]
fn increase_the_frequency_decrease_number_peaks() {
    let sound1 = run_gb_with_settings(1, 1, 0, 0);
    let sound2 = run_gb_with_settings(1, u8::MAX as u16, 0, 0);

    let total1: i16 = sound1.iter().sum();
    let total2: i16 = sound2.iter().sum();

    println!("total 1: {}", &total1);
    println!("total 2: {}", &total2);

    assert!(total1 > total2);
}

#[test]
fn increase_duty_increases_peaks() {
    let sound1 = run_gb_with_settings(1, u8::MAX as u16, 0, 0);
    let sound2 = run_gb_with_settings(1, u8::MAX as u16, 3, 0);

    let total1: i16 = sound1.iter().sum();
    let total2: i16 = sound2.iter().sum();

    println!("total 1: {}", &total1);
    println!("total 2: {}", &total2);

    println!("sound1 {:?}", sound1);
    println!("sound2 {:?}", sound2);

    assert!(total1 < total2);
}

#[test]
fn period_reduces_volume() {
    let audio = run_gb_with_settings(15, 1028, 1, 1);

    // and remove the 0 values
    let mut no_zeros: Vec<i16> = audio.into_iter().filter(|v| *v != 0).collect();

    // If we remove consecutive duplicates
    no_zeros.dedup();

    // we should have 2 values decreasing in value
    assert!(no_zeros.len() == 2);
    assert!(no_zeros[0] > no_zeros[1]);
}
