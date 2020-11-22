pub struct ALU<'a> {
    audio_callback: Box<dyn FnMut(i16) + 'a>,
    cycles_to_next_sample: i32,
}

const SAMPLE_RATE: i32 = 48000; // Hz
const CYCLES_PER_SECOND: i32 = 4194304;
const CYCLES_PER_SAMPLE: i32 = CYCLES_PER_SECOND / SAMPLE_RATE;

impl<'a> ALU<'a> {
    pub fn new<F>(audio_callback: F) -> ALU<'a>
    where
        F: FnMut(i16) + 'a,
    {
        ALU {
            audio_callback: Box::new(audio_callback),
            cycles_to_next_sample: CYCLES_PER_SAMPLE,
        }
    }

    pub fn tick(&mut self, tick: u32, _memory: &mut Vec<u8>) {
        // decrease the cycles_to_next_sample
        self.cycles_to_next_sample = self.cycles_to_next_sample - tick as i32;

        // if the cycles are less than 0 then emit a value, reset the count
        if self.cycles_to_next_sample <= 0 {
            self.audio_callback.as_mut()(0);
            self.cycles_to_next_sample = CYCLES_PER_SAMPLE + self.cycles_to_next_sample;
        }
    }
}
