//! Enables a PIT timer to test interrupts
//!
//! Success criteria: the LED is on for 250ms,
//! then off for 250ms, then on for 250ms, then off
//! for 250ms... This is observable using a scope, or
//! a really good eye, finger, and stopwatch.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::pit;
use bsp::interrupt;
use cortex_m_rt::{entry, interrupt};
use embedded_hal::timer::CountDown;
use imxrt1060evk_bsp as bsp;

static mut TIMER: Option<pit::PIT<pit::channel::_3>> = None;

#[interrupt]
unsafe fn PIT() {
    // If this timer expired, `wait()` returns `Ok(())`
    // after re-arming the timer. On the other hand,
    // `wait()` returns `Err(WouldBlock)` if the timer
    // didn't expire. Use this to change state.
    let _ = TIMER.as_mut().unwrap().wait();
}

#[entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();
    let pins = bsp::pins::from_pads(periphs.iomuxc);
    let (_, ipg_hz) = periphs.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut periphs.ccm.handle,
        &mut periphs.dcdc,
    );
    periphs.ccm.pll2.set(
        &mut periphs.ccm.handle,
        [
            Some(bsp::hal::ccm::pll2::MHZ_352),
            Some(bsp::hal::ccm::pll2::MHZ_594),
            Some(bsp::hal::ccm::pll2::MHZ_396),
            Some(bsp::hal::ccm::pll2::MHZ_297),
        ],
    );
    periphs.ccm.pll3.set(
        &mut periphs.ccm.handle,
        [
            Some(bsp::hal::ccm::pll3::MHZ_720),
            Some(bsp::hal::ccm::pll3::MHZ_664),
            Some(bsp::hal::ccm::pll3::MHZ_508),
            Some(bsp::hal::ccm::pll3::MHZ_454),
        ],
    );

    let mut cfg = periphs.ccm.perclk.configure(
        &mut periphs.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_3,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );

    let (_, _, _, mut timer) = periphs.pit.clock(&mut cfg);
    timer.set_interrupt_enable(true);
    unsafe {
        TIMER = Some(timer);
        TIMER
            .as_mut()
            .unwrap()
            .start(core::time::Duration::from_millis(250));
        cortex_m::peripheral::NVIC::unmask(interrupt::PIT);
    }
    let mut led = bsp::configure_led(pins.d4);
    loop {
        led.toggle();
        cortex_m::asm::wfi();
    }
}
