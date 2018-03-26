#![deny(unsafe_code)]
#![no_std]

extern crate aux5;

use aux5::prelude::*;
use aux5::{Delay, Leds};

fn main() {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let step_period = 50_u16;
    loop {
        for current in 0..8 {
            let next = (current + 1) % 8;
            leds[next].on();
            delay.delay_ms(step_period);
            leds[current].off();
            delay.delay_ms(step_period);
        }
    }
}
