#![no_main]
#![no_std]

use embedded_hal::delay::DelayNs;
use microbit::hal::{self, qdec};
use mb2_rotary_encoder::{QPin, QIn, QLed};

struct Nrf52RotaryEncoder {
    qdec: qdec::Qdec<QIn, QLed>,
    count: isize,
}

impl Nrf52RotaryEncoder {
    fn new(
        qdec: hal::pac::QDEC,
        a: QPin,
        b: QPin,
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
    Nrf52RotaryEncoder::new,
    |r: &mut Nrf52RotaryEncoder| r.update(),
    |r: &mut Nrf52RotaryEncoder, _| r.count,
    2000,
}
