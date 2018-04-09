#![deny(unsafe_code)]
#![no_std]

extern crate aux5;

use aux5::prelude::*;
use aux5::{Delay, Leds};

fn main() {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let step_period = 100_u16;
    let mut step = 0;
    let intervals = [(5,1), (1,6), (6,2), (2,7), (7,3), (3,0), (0,4), (4,1),
      (1,5), (5,2), (2,6), (6,3), (3,7), (7,4), (4,0), (0,5)];

    loop {
        for &(off, on) in intervals.iter() {
            leds[off].off();
            delay.delay_ms(step_period);
            leds[on].on();
            delay.delay_ms(step_period);
            delay.delay_ms(step_period);
        }
    }
}
