//! Turns on the evk led

#![no_std]
#![no_main]

extern crate panic_halt;

use imxrt1060evk_bsp::{rt::entry, ral, hal, ral::write_reg, ral::gpio};

fn set_pin_9(gpio: &gpio::RegisterBlock) {
    write_reg!(gpio, gpio, DR, 1<<9);
}

fn clear_pin_9(gpio: &gpio::RegisterBlock) {
    write_reg!(gpio, gpio, DR, 0);
}

#[entry]
fn main() -> ! {
    //TODO  Enable some clocks (eq to SDKs CLOCK_Enable(gpio1)) to make this work
    let ccm = ral::ccm::CCM::take().unwrap();
    let ccm_analog = ral::ccm_analog::CCM_ANALOG::take().unwrap();
    let dcdc = ral::dcdc::DCDC::take().unwrap();
    let clk = 600_000_000;
    let (_arm_clk, _ipg_clk) = hal::ccm::set_arm_clock(clk, &ccm, &ccm_analog, &dcdc);
   
    let gpio1 = gpio::GPIO1::take().unwrap();
    write_reg!(gpio, gpio1, GDIR, 1<<9);
    write_reg!(gpio, gpio1, DR, 0);
    let mut n = 0;
    loop {
        if n == 0 {
            set_pin_9(&gpio1);
        } else {
            clear_pin_9(&gpio1);
        }
        n = (n + 1) & 0x1;
        for _i in 0..10000 {
            core::sync::atomic::spin_loop_hint();
        }
    }
}
