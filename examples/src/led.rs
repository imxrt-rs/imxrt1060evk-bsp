//! The example simply enables the LED
//!
//! Success criteria: the LED turns on!

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt::entry;
use cortex_m::asm::wfi;
use imxrt1060evk_bsp as bsp;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut led = bsp::configure_led(&mut peripherals.gpr, peripherals.pins.d4);

    loop {
        led.set_high().unwrap();
        wfi();
    }
}
