#![deny(unsafe_code)]
#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

pub use f3::{
    hal::{delay::Delay, prelude},
    led::Leds,
};

mod init;
mod led_wheel;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = init::init();

    led_wheel::blink(&mut leds, &mut delay, 10, 100);

    loop {
//        led_wheel::all_leds_off(&mut leds);
//        delay.delay_ms(500_u16);
//
//        led_wheel::blurred_cycle(&mut leds, &mut delay, 3, 1000);
//
//        led_wheel::all_leds_off(&mut leds);
//        delay.delay_ms(500_u16);
//
//        led_wheel::blink(&mut leds, &mut delay, 3, 1000);
//
//        led_wheel::all_leds_off(&mut leds);
//        delay.delay_ms(500_u16);
//
//        led_wheel::on_then_off_cycle(&mut leds, &mut delay, 6, 500);

//        led_wheel::all_leds_off(&mut leds);
//        delay.delay_ms(500_u16);
//
//        led_wheel::on_then_off_cycle_2(&mut leds, &mut delay, 8, 500);

        led_wheel::off_cycle(&mut leds, &mut delay, 3, 1000);
    }
}
