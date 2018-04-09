#![deny(unsafe_code)]
#![no_std]

extern crate aux5;

use aux5::prelude::*;
use aux5::{Delay, Leds};

fn main() {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let mut step_period = 100.0;
    let mut decreasing = true;
    let intervals = [(5,1), (1,6), (6,2), (2,7), (7,3), (3,0), (0,4), (4,1),
      (1,5), (5,2), (2,6), (6,3), (3,7), (7,4), (4,0), (0,5)];
    loop {
        for &(off, on) in intervals.iter() {
            leds[off].off();
            let wait = step_period as u16;
            delay.delay_ms(wait);
            leds[on].on();
            delay.delay_ms(wait);
            delay.delay_ms(wait);
            if (step_period <= 20.0) {
                decreasing = false
            } else if (step_period >= 100.0) {
                decreasing = true
            }
            if decreasing {
                step_period = step_period * 0.99;
            } else {
                step_period = step_period * 1.01;
            }
        }
    }
}
