#[derive(Clone, Copy)]
pub enum DutyCycle {
    Zero,
    One,
    Two,
    Three,
}

pub fn get_duty(duty: DutyCycle, pos: u8) -> u8 {
    let index = pos as usize;
    match duty {
        DutyCycle::Zero => [0, 0, 0, 0, 0, 0, 0, 1][index],
        DutyCycle::One => [1, 0, 0, 0, 0, 0, 0, 1][index],
        DutyCycle::Two => [1, 0, 0, 0, 0, 1, 1, 1][index],
        DutyCycle::Three => [0, 1, 1, 1, 1, 1, 1, 0][index],
    }
}

impl From<u8> for DutyCycle {
    fn from(val: u8) -> Self {
        match val {
            0 => DutyCycle::Zero,
            1 => DutyCycle::One,
            2 => DutyCycle::Two,
            3 => DutyCycle::Three,
            other => {
                panic!("Could not set duty cycle from value {}", other)
            }
        }
    }
}
