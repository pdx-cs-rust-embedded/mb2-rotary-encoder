#[macro_export]
macro_rules! main {
    ($init:expr, $poll:expr, $update:expr, $delay:expr $(,)?) => {
        use panic_rtt_target as _;
        use rtt_target::{rprintln, rtt_init_print};

        use cortex_m_rt::entry;
        use microbit::{
            board::Board,
            hal::{Delay, prelude::*},
        };

        #[entry]
        fn main() -> ! {
            rtt_init_print!();
            let board = Board::take().unwrap();
            let mut delay = Delay::new(board.SYST);
            let qdec = board.QDEC;

            #[cfg(feature="floating")]
            let (mut pin_a, mut pin_b) = (
                board.edge.e08.into_floating_input().degrade(),
                board.edge.e09.into_floating_input().degrade(),
            );

            #[cfg(not(feature="floating"))]
            let (mut pin_a, mut pin_b) = (
                board.edge.e08.into_pullup_input().degrade(),
                board.edge.e09.into_pullup_input().degrade(),
            );

            let a = pin_a.is_high().unwrap() as u8;
            let b = pin_b.is_high().unwrap() as u8;
            rprintln!("initial: {}{}", a, b);

            let mut rotary = ($init)(qdec, pin_a, pin_b);

            let mut count = 0;

            loop {
                delay.delay_us($delay);
                ($poll)(&mut rotary);
                let new_count = ($update)(&mut rotary, count);
                if new_count != count {
                    rprintln!("{}", new_count);
                    count = new_count;
                }
            }
        }
    };
}
