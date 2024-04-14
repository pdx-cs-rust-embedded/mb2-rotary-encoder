#![no_main]
#![no_std]

use rotary_encoder_hal::{Rotary, Direction};

fn update<A, B>(r: &mut Rotary<A, B>, count: isize) -> isize
    where A: InputPin, B: InputPin,
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
    1000u16,
}
