#![no_main]
#![no_std]

use embedded_hal::{digital::InputPin, delay::DelayNs};
use rotary_encoder_embedded::{RotaryEncoder, Direction, standard::StandardMode};

fn update<D, C>(r: &mut RotaryEncoder<StandardMode, D, C>, count: isize) -> isize
    where D: InputPin, C: InputPin,
{
    let delta = match r.update() {
        Direction::Clockwise => 1,
        Direction::Anticlockwise => -1,
        Direction::None => 0,
    };
    count + delta
}

mb2_rotary_encoder::main! {
    |pin_a, pin_b| RotaryEncoder::new(pin_b, pin_a).into_standard_mode(),
    |r: &mut RotaryEncoder<StandardMode, _, _>| r.update(),
    update,
    1111,
}



