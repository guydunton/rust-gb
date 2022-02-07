use super::Channel;
use super::{timer::TickResult, Timer};

#[allow(clippy::upper_case_acronyms)]
pub struct ALU<'a> {
    audio_callback: Box<dyn FnMut(i16) + 'a>,
    sample_timer: Timer,
    square_channel_1: Channel,
}

const SAMPLE_RATE: i32 = 44100; // Hz
const CYCLES_PER_SECOND: i32 = 4194304;
const CYCLES_PER_SAMPLE: i32 = CYCLES_PER_SECOND / SAMPLE_RATE;
// const CYCLES_PER_LENGTH: i32 = CYCLES_PER_SECOND / 256;

impl<'a> ALU<'a> {
    pub fn new<F>(audio_callback: F) -> ALU<'a>
    where
        F: FnMut(i16) + 'a,
    {
        let mut sample_timer = Timer::new();
        sample_timer.start(CYCLES_PER_SAMPLE);

        ALU {
            audio_callback: Box::new(audio_callback),
            sample_timer,
            square_channel_1: Channel::new(),
        }
    }

    pub fn tick(&mut self, tick: u32, memory: &mut Vec<u8>) {
        self.square_channel_1.tick(tick, memory);

        // if the cycles are less than 0 then emit a value, reset the count
        if self.sample_timer.tick(tick) == TickResult::Ticked {
            let volume = self.square_channel_1.get_volume();
            self.audio_callback.as_mut()(volume);
        }
    }
}
