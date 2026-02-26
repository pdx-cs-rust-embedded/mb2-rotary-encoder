#![no_main]
#![no_std]

use embedded_hal::delay::DelayNs;
use microbit::hal::{self, gpio::*, qdec};

struct Mb2RotaryEncoder {
    qdec: qdec::Qdec,
    count: isize,
}

impl Mb2RotaryEncoder {
    fn new(
        qdec: hal::pac::QDEC,
        a: Pin<Input<PullUp>>,
        b: Pin<Input<PullUp>>,
    ) -> Self {
        let pins = qdec::Pins { a, b, led: None };
        let qdec = qdec::Qdec::new(qdec, pins, qdec::SamplePeriod::_256us);
        qdec.debounce(true);
        qdec.enable();
        Self {
            qdec,
            count: 0,
        }
    }

    fn update(&mut self) {
        self.count += self.qdec.read() as isize;
    }
}

mb2_rotary_encoder::main! {
    Mb2RotaryEncoder::new,
    |r: &mut Mb2RotaryEncoder| r.update(),
    |r: &mut Mb2RotaryEncoder, _| r.count,
    2000,
}
