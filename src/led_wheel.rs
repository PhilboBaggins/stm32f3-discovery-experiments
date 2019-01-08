#![deny(unsafe_code)]

use f3::{
    hal::{delay::Delay},
    led::Leds,
};

use f3::hal::prelude::*;

const NUM_LEDS: u16 = 8;

// ----------------------------------------
// Patterns
// ----------------------------------------

#[allow(dead_code)]
pub fn blurred_cycle(leds: &mut Leds, delay: &mut Delay, iterations: u16, period_ms: u16) {
    for _ in 0..iterations {
        for curr_led in 0..NUM_LEDS {
            let next_led = (curr_led + 1) % NUM_LEDS;
            let next_next_led = (next_led + 1) % NUM_LEDS;

            leds[next_led as usize].on();
            delay.delay_ms(period_ms / 3 / NUM_LEDS as u16);

            leds[next_next_led as usize].on();
            delay.delay_ms(period_ms / 3 / NUM_LEDS as u16);

            leds[curr_led as usize].off();
            delay.delay_ms(period_ms / 3 / NUM_LEDS as u16);
        }
    }
}

#[allow(dead_code)]
pub fn blink(leds: &mut Leds, delay: &mut Delay, iterations: u16, period_ms: u16) {
    for _ in 0..iterations {
        all_leds_on(leds);
        delay.delay_ms(period_ms / 2);

        all_leds_off(leds);
        delay.delay_ms(period_ms / 2);
    }
}

#[allow(dead_code)]
pub fn on_then_off_cycle(leds: &mut Leds, delay: &mut Delay, iterations: u16, period_ms: u16) {
    for _ in 0..iterations {
        for curr_led in 0..NUM_LEDS {
            leds[curr_led as usize].on();
            delay.delay_ms(period_ms / 2 / NUM_LEDS as u16);
        }

        for curr_led in (0..NUM_LEDS).rev() {
            leds[curr_led as usize].off();
            delay.delay_ms(period_ms / 2 / NUM_LEDS as u16);
        }
    }
}

#[allow(dead_code)]
pub fn on_then_off_cycle_2(leds: &mut Leds, delay: &mut Delay, iterations: u16, period_ms: u16) {
    for i in 0..iterations {
        for curr_led in 0..NUM_LEDS {
            let curr_led = (curr_led + i) % NUM_LEDS;
            leds[curr_led as usize].on();
            delay.delay_ms(period_ms / 2 / NUM_LEDS);
        }

        for curr_led in (0..NUM_LEDS).rev() {
            let curr_led = (curr_led + i) % NUM_LEDS;
            leds[curr_led as usize].off();
            delay.delay_ms(period_ms / 2 / NUM_LEDS as u16);
        }
    }
}

#[allow(dead_code)]
pub fn off_cycle(leds: &mut Leds, delay: &mut Delay, iterations: u16, period_ms: u16) {
    all_leds_on(leds);

    for _ in 0..iterations {
        for curr_led in 0..NUM_LEDS {
            leds[curr_led as usize].off();
            delay.delay_ms(period_ms / NUM_LEDS);
            leds[curr_led as usize].on();
        }
    }

    all_leds_off(leds);
}

// ----------------------------------------
// Helper functions
// ----------------------------------------

#[allow(dead_code)]
pub fn all_leds_on(leds: &mut Leds) {
    for led in leds.iter_mut() {
        led.on();
    }
}

#[allow(dead_code)]
pub fn all_leds_off(leds: &mut Leds) {
    for led in leds.iter_mut() {
        led.off();
    }
}
