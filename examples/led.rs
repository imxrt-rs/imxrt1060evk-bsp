//! The example simply enables the LED
//!
//! Success criteria: the LED turns on!

#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m::asm::wfi;
use imxrt1060evk_bsp as bsp;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let peripherals = bsp::Peripherals::take().unwrap();
    let pins = bsp::pins::from_pads(peripherals.iomuxc);
    let mut led = bsp::configure_led(pins.d4);

    loop {
        led.set_low().unwrap();
        wfi();
    }
}
