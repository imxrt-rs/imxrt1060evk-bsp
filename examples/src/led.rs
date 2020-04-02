//! The example simply enables the LED
//!
//! Success criteria: the LED turns on!

#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_semihosting;

use imxrt1060evk_bsp as bsp;
use bsp::rt::entry;
use bsp::hal::gpio::IntoGpio;
use cortex_m::asm::{wfi, nop};


use embedded_hal::digital::v2::ToggleableOutputPin;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut led = bsp::configure_led(&mut peripherals.gpr, peripherals.pins.d4.alt5());

    // HACK set the VTOR value and enable interrupts
    unsafe { 
//        //const VTOR: *mut u32 = 0xE000ED08 as *mut u32;
//        //core::ptr::write_volatile(VTOR, 0x00000004);
//
        cortex_m::interrupt::enable() 
    };
//
    loop {
        led.toggle().unwrap();
        for _n in 0..1000 {
            core::sync::atomic::spin_loop_hint();
        }

    }
}
