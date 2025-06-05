#![no_main]
#![no_std]

use embedded_hal::{digital::InputPin, delay::DelayNs};
use rotary_encoder_hal::{Direction, Phase, Rotary};

fn update<A, B, P>(r: &mut Rotary<A, B, P>, count: isize) -> isize
    where A: InputPin, B: InputPin, P: Phase,
{
    let delta = match r.update().map_err(|_| "update error").unwrap() {
        Direction::Clockwise => 1,
        Direction::CounterClockwise => -1,
        Direction::None => 0,
    };
    count + delta
}

mb2_rotary_encoder::main! {
    Rotary::new,
    |_| (),
    update,
    1000,
}
