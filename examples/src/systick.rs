//! Implements the systick exception and uses it
//! to toggle the LED every second.
//!
//! Success critera: the LED is on for 1 second, then off
//! for 1 second, then on for 1 second, then off for 1 second.

#![no_std]
#![no_main]
extern crate panic_halt;

use bsp::rt;
use embedded_hal::digital::v2::ToggleableOutputPin;
use imxrt1060evk_bsp as bsp;

const LED_PERIOD_MS: u32 = 1_000;

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let mut led: bsp::LED = bsp::configure_led(&mut p.gpr, p.pins.d4.alt5());

    loop {
        bsp::delay(LED_PERIOD_MS);
        led.toggle().unwrap();
    }
}
