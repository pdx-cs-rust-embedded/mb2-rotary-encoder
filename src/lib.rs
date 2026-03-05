#![no_std]

use microbit::hal::gpio::*;

#[cfg(feature = "floating")]
pub type QIn = Floating;
#[cfg(not(feature = "floating"))]
pub type QIn = PullUp;

pub type QLed = Floating;

pub type QPin = Pin<Input<QIn>>;

pub mod boilerplate;
pub mod lib_bad;
