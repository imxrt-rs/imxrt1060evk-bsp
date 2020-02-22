//! Turns on the evk led

#![no_std]
#![no_main]

extern crate panic_halt;

use imxrt1060evk_bsp::{rt::entry, ral::gpio, ral::write_reg};

fn toggle_pin_9(gpio: &gpio::RegisterBlock) {
    write_reg!(gpio, gpio, DR_TOGGLE, 1<<9);
}


#[entry]
fn main() -> ! {
    //TODO  Enable some clocks (eq to SDKs CLOCK_Enable(gpio1)) to make this work
    let gpio1 = gpio::GPIO1::take().unwrap();
    write_reg!(gpio, gpio1, GDIR, 1<<9);
    loop {
        toggle_pin_9(&gpio1);
        for _i in 0..10000 {
            core::sync::atomic::spin_loop_hint();
        }
    }
}
