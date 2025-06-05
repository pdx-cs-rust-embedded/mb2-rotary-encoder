#[macro_export]
macro_rules! main {
    ($init:expr, $poll:expr, $update:expr, $delay:expr $(,)?) => {
        use panic_rtt_target as _;
        use rtt_target::{rprintln, rtt_init_print};

        use cortex_m_rt::entry;
        use microbit::{board::Board, hal::{prelude::*, Delay}};

        #[entry]
        fn main() -> ! {
            rtt_init_print!();
            let board = Board::take().unwrap();
            let mut delay = Delay::new(board.SYST);
            let _pin_s = board.edge.e16.into_pullup_input().degrade();
            let pin_a = board.edge.e08.into_pullup_input().degrade();
            let pin_b = board.edge.e09.into_pullup_input().degrade();
            let mut rotary = ($init)(pin_a, pin_b);
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
