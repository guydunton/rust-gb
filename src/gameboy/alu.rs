pub struct ALU {
    buffer: Vec<i16>,
    cycles_to_next_sample: i32,
}

const SAMPLE_RATE: i32 = 48000; // Hz
const CYCLES_PER_SECOND: i32 = 4194304;
const SAMPLES_PER_FRAME: i32 = SAMPLE_RATE / 60; // 800
const CYCLES_PER_SAMPLE: i32 = CYCLES_PER_SECOND / (60 * SAMPLES_PER_FRAME);

impl ALU {
    pub fn new() -> ALU {
        let mut buffer = Vec::new();
        buffer.reserve(SAMPLES_PER_FRAME as usize + 3);
        ALU {
            buffer,
            cycles_to_next_sample: CYCLES_PER_SAMPLE,
        }
    }

    pub fn tick(&mut self, tick: u32, _memory: &mut Vec<u8>) {
        // decrease the cycles_to_next_sample
        self.cycles_to_next_sample = self.cycles_to_next_sample - tick as i32;

        // if the cycles are less than 0 then emit a value, reset the count
        if self.cycles_to_next_sample <= 0 {
            if self.buffer.len() >= 803 {
                self.buffer.clear();
            }

            self.buffer.push(0);
            self.cycles_to_next_sample = CYCLES_PER_SAMPLE + self.cycles_to_next_sample;
        }
    }

    pub fn get_sample(&self) -> Vec<i16> {
        if self.buffer.len() > 8 {
            let mut result = vec![0; 800];
            result.copy_from_slice(&self.buffer[0..800]);
            result
        } else {
            self.buffer.clone()
        }
    }
}
