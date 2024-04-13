# mb2-rotary-encoder: rotary encoder eval and crate bakeoff
Bart Massey 2024

This crate is part of an evaluation of the use of mechanical
rotary encoder knobs with the BBC MicroBit v2.

## Device and Wiring

I purchased ten inexpensive rotary encoders with knobs and
mounting hardware, branded *Konohan,* from Amazon
<https://www.amazon.com/dp/B09KNC1J6H>. They arrived
2024-04-12.

This part bears no markings. It is a 20-count rotary encoder
with 20 detents and an independent momentary-contact switch
for shaft press.  It is advertised as "compatible with
Arduino EC11", which seems to be true: the pin geometry and
function seem correct.

The rotary encoder pins are A C B where C is the center pin.
(A is sometimes referred to as "clock" and B as "data" in
the literature, though the pins are more or less symmetric.)

I wired the rotary with C at GND and A and B connected to
pins 8 (GPIO1) and 9 (GPIO2) on the nRF52833 edge card. I
connected the push switch to pin 16 (GPIO3) on the nRF52833
edge card and to GND. All three GPIOs were configured as
pull-up inputs.

With this wiring, the detent rest is at 11. The output is
effectively a 2-bit Gray code.

    Clockwise step:
      A B
      1 1
      1 0
      0 0
      0 1

    Counterclockwise step:
      A B
      1 1
      0 1
      0 0
      1 0

Like all mechanical rotary encoders, this device is prone to
severe bouncing, missed steps, etc.

## Project Structure

The software goal was to build separate binaries to evaluate
various Rust library crates for handling rotary decoding. To
this end, a "boilerplate" `main` macro was constructed that
allowed simple substitution of various polled decoding
routines.

## Library Crates

Having set up, various library crates were evaluated.

### `bad`

I started out with my own super-cheese solution for decoding.
The library sources is in `src/lib_bad.rs`; the app source
is in `src/bin/bad.rs`.

The library works surprisingly well: it wants a 2 ms poll,
and maintains an internal count.

### `rotary-encoder-embedded`

[`rotary-encoder-embedded`](https://crates.io/crates/rotary-encoder-embedded)
is a lightweight library with good documentation, and knob
velocity support. It was last updated about 1.5 years ago.

The library is relatively easy to use: it wants a 1.111 ms
poll, and maintains a step enum.
  
App source code is in `src/bin/rotary-encoder-embedded.rs`.
The app occasionally drops or doubles clicks, but is
otherwise fine.

[`rotary-encoder-hal`](https://crates.io/crates/rotary-encoder-hal)
is a lightweight library with reasonable documentation, last
updated about 2.5 years ago.

[`sb-rotary-encoder`](https://crates.io/crates/sb-rotary-encoder)
is a fancy library with solid documentation, knob velocity
support. Last updated about a year ago.
  
### Not Evaluated

Other libraries found on `crates.io` didn't seem worth
poking at right now.

* `rotary-encoder` is a placeholdery-looking 0.1.0 library
  with no README from over 7 years ago.

* `embtk-rotary-encoder` is a five-year-old library with no
  README.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
