#![no_main]
#![no_std]

use embedded_hal::delay::DelayNs;
use mb2_rotary_encoder::{lib_bad::Rotary, main};

main! {
    |_, pin_a, pin_b| Rotary::new(pin_a, pin_b),
    |r: &mut Rotary| r.poll(),
    |r: &mut Rotary, _| r.count(),
    2000,
}
