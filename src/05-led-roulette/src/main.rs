#![deny(unsafe_code)]
#![no_std]

extern crate aux5;

use aux5::prelude::*;
use aux5::{Delay, Leds};

fn main() {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let step_period = 50_u16;
    let mut step = 0;
    let intervals = [(0,2), (2,4), (4,6), (6,8), (8,10), (10,12), (12,14)];

    loop {
        if step == 0 || step == 14 || step == 15 {
            leds[0].on();
        } else {
            leds[0].off();
        }
        let index = 1;
        for (index, &(start, stop)) in intervals.iter().enumerate() {
            if step >= start && step <= stop{
                leds[index+1].on();
            } else {
                leds[index+1].off();
            }
        }
        delay.delay_ms(step_period);
        step = (step + 1) % 16;
    }
}
