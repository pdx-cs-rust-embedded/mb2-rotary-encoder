use embedded_hal::digital::InputPin;
use microbit::hal::gpio::*;

struct Encoder {
    pin_a: Pin<Input<PullUp>>,
    pin_b: Pin<Input<PullUp>>,
}

impl Encoder {
    fn new(
        pin_a: Pin<Input<PullUp>>,
        pin_b: Pin<Input<PullUp>>,
    ) -> Self {
        Self { pin_a, pin_b }
    }

    fn read(&mut self) -> u8 {
        let a = self.pin_a.is_high().unwrap();
        let b = self.pin_b.is_high().unwrap();
        ((a as u8) << 1) | b as u8
    }
}

pub struct Rotary {
    encoder: Encoder,
    count: isize,
    state: u8,
}

impl Rotary {
    pub fn new(
        pin_a: Pin<Input<PullUp>>,
        pin_b: Pin<Input<PullUp>>,
    ) -> Self {
        let encoder = Encoder::new(pin_a, pin_b);
        Self { encoder, state: 0b11, count: 0 }
    }

    pub fn poll(&mut self) {
        let next_state = self.encoder.read();
        if next_state == self.state {
            return;
        }
        if next_state == 0b11 {
            match self.state {
                0b01 => self.count += 1,
                0b10 => self.count -= 1,
                _ => (),
            }
        }
        self.state = next_state;
    }

    pub fn count(&self) -> isize {
        self.count
    }
}
