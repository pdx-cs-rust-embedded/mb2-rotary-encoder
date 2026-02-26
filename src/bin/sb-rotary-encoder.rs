#![no_main]
#![no_std]

use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::hal::gpio::*;

// Number of pulses required for one step. 4 is a typical value for encoders with detents.
const PULSE_DIVIDER: i32 = 4;

struct Encoder {
    pin_a: Pin<Input<PullUp>>,
    pin_b: Pin<Input<PullUp>>,
}

impl Encoder {
    fn new(pin_a: Pin<Input<PullUp>>, pin_b: Pin<Input<PullUp>>) -> Self {
        Self { pin_a, pin_b }
    }

    fn read(&mut self) -> (bool, bool) {
        let a = self.pin_a.is_high().unwrap();
        let b = self.pin_b.is_high().unwrap();
        (a, b)
    }
}

struct Mb2RotaryEncoder {
    encoder: sb_rotary_encoder::RotaryEncoder,
    pins: Encoder,
    count: isize,
}

impl Mb2RotaryEncoder {
    fn new(pin_a: Pin<Input<PullUp>>, pin_b: Pin<Input<PullUp>>) -> Self {
        let encoder = sb_rotary_encoder::RotaryEncoder::new();
        let pins = Encoder::new(pin_a, pin_b);
        Self {
            encoder,
            pins,
            count: 0,
        }
    }

    fn update(&mut self) {
        let (input_a, input_b) = self.pins.read();
        let maybe_event = self.encoder.update(input_a, input_b, None, PULSE_DIVIDER);
        if let Some(event) = maybe_event {
            self.count = event.value() as isize;
        }
    }
}

mb2_rotary_encoder::main! {
    |pin_a, pin_b| Mb2RotaryEncoder::new(pin_b, pin_a),
    |r: &mut Mb2RotaryEncoder| r.update(),
    |r: &mut Mb2RotaryEncoder, _| r.count,
    1000,
}
