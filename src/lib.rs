//! A Rust board support package (BSP) for the MIMXRT1060-EVK
//!
//! As of this writing, the BSP is very primitive. It exposes
//! only the LED, and it configures facilities for logging over UART to
//! the on-board LPC4432 which acts as a debug probe.
//!
//! Otherwise, it simply forwards components from the HAL for your
//! own usage. This will be addressed as the HAL becomes more developed.
//!
//! The BSP does assume some facilities of the processor:
//!
//! - it registers the `SysTick` exception handler, and configures
//!   SYSTICK for a 1ms interrupt.
//!
//! These peripherals and capabilities are not exported from the BSP.
//! If a user also registers a `SysTick` or `USB_OTG1` handler, it may
//! result in a duplicate definition error.
//!
//! ## Re-exports
//!
//! The BSP re-exports the following:
//!
//! - the `cortex-m-rt` crate, as `rt`
//! - the `imxrt-ral` crate, as `ral`
//! - the `imxrt-hal` crate, as `hal`
//!
//! See the accompanying documentation of each crate for more
//! information.
//!
//! For simplicity, there may be other choice APIs from either crate that
//! are re-exported in the BSP namespace.
//!
//! ## Examples
//!
//! See the `examples` directory for build-able, run-able
//! examples. The examples utilize this BSP crate to blink LEDs,
//! establish timers, and log data over UART.
//!
//! ## Notice of alpha status
//!
//! We've made some assumptions in this MVP BSP.
//!
//! - SYSTICK and delay implementation is very naive. Do not run for 49
//!   continuous days, or risk a millisecond counter wrap-around.

#![no_std]

pub use imxrt1060evk_pins as pins;

#[cfg(feature = "systick")]
mod systick;
#[cfg(feature = "systick")]
pub use systick::SysTick;

pub use hal::ral::interrupt;
// `rtic` expects these in the root.
#[doc(hidden)]
#[cfg(feature = "rtic")]
pub use hal::ral::{interrupt as Interrupt, NVIC_PRIO_BITS};

pub use hal::Peripherals;
pub use imxrt_hal as hal;

/// The LED
///
/// See [`configure_led`](fn.configure_led.html) to prepare the LED.
pub type Led = hal::gpio::GPIO<pins::D4, hal::gpio::Output>;

/// Configure the board's LED
///
/// Returns a GPIO that's physically tied to the LED. Use the returned handle
/// to drive the LED.
pub fn configure_led(mut pin: pins::D4) -> Led {
    use hal::iomuxc::*;

    const LED_PIN_CFG: Config = Config::modify()
        .set_pull_keep_select(PullKeepSelect::Keeper)
        .set_pull_keep(PullKeep::Disabled)
        .set_speed(Speed::Fast)
        .set_drive_strength(DriveStrength::R0_6);

    configure(&mut pin, LED_PIN_CFG);

    let led = hal::gpio::GPIO::new(pin);
    led.output()
}
