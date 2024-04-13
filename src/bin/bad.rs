#![no_main]
#![no_std]

use mb2_rotary_encoder::{main, lib_bad::Rotary};

main! {
    Rotary::new,
    |r: &mut Rotary| r.poll(),
    |r: &mut Rotary, _| r.count(),
    2000u16,
}



