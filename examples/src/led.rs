//! The example simply enables the LED
//!
//! Success criteria: the LED turns on!

#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_semihosting;

use bsp::rt::entry;
use imxrt1060evk_bsp as bsp;
use cortex_m_semihosting::{debug, hio};
use core::fmt::Write;
use bsp::hal::gpio::IntoGpio;


use embedded_hal::digital::v2::ToggleableOutputPin;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut led = bsp::configure_led(&mut peripherals.gpr, peripherals.pins.d4.alt5());
    let mut stdout = hio::hstdout().unwrap();

    write!(stdout, "init done, looping").unwrap();
    loop {
        write!(stdout, "toggling").unwrap();
        led.toggle().unwrap();
        for i in 0..1000 {
            core::sync::atomic::spin_loop_hint();
        }
    }
}
