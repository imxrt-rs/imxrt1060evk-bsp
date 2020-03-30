//! A Rust board support package (BSP) for the MIMXRT1060-EVK
//!
//! As of this writing, the BSP is very primitive. It exposes
//! only the LED, and it configures facilities for logging over UART.
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
//! - the `imxrt-rt` crate, as `rt`
//! - the `imxrt-ral` crate, as `ral`
//! - the `imxrt-hal` crate, as `hal`
//!
//! See the accompanying documentation of each crate for more
//! information.
//!
//! For simplicity, there may be other choice APIs from either crate that
//! are re-exported in the BSP namespace.
//!
//! ## Arduino Connector and Alternative Functions
//!
//! The sparse table below describes the Arduino header pins, the pad ID, and
//! some of the notable alternative functionalities for each pin. We add entries
//! to the table as we add capabilities to the underlying HAL crate.
//! Contributions to complete this table are welcome! If a pad's alternatives
//! are not listed here, consult the iMXRT1060 reference manual.
//!
//! | Pin   | Pad ID   |  Alt0    |  Alt1        |  Alt2        |  Alt3     |  Alt4        |  Alt5            |  Alt6        |  Alt7   |  Alt8   |  Alt9   |
//! | ----- | -------- | -------- | ------------ | ------------ | --------- | ------------ | ---------------- | ------------ | ------- | ------- | ------- |
//! |  D0   |`AD_B1_07`|          |              |  `UART6_RX`  |           |`FlexPWM1_1_X`|                  |              |         |         |         |
//! |  D1   |`AD_B1_06`|          |              |  `UART6_TX`  |           |`FlexPWM1_0_X`|                  |              |         |         |         |
//! |  D2   |`AD_B0_11`|          |`FlexPWM4_2_A`|              |           |              |                  |              |         |         |         |
//! |  D3   |`AD_B1_08`|          |`FlexPWM4_2_B`|              |           |              |                  |              |         |         |         |
//! |  D4   |`AD_B0_09`|          |`FlexPWM2_0_A`|              |           |              |                  |              |         |         |         |
//! |  D5   |`AD_B0_10`|          |`FlexPWM2_1_A`|              |           |              |                  |              |         |         |         |
//! |  D6   |`AD_B1_02`|          |              |`FlexPWM2_2_A`|           |              |                  |              |         |         |         |
//! |  D7   |`AD_B1_03`|          |              |  `UART4_RX`  |           |              |                  |`FlexPWM1_3_B`|         |         |         |
//! |  D8   |`AD_B0_03`|          |              |  `UART4_TX`  |           |              |                  |`FlexPWM1_3_A`|         |         |         |
//! |  D9   |`AD_B0_02`|          |              |`FlexPWM2_2_B`|           |              |                  |              |         |         |         |
//! |  D10  |`SD_B0_01`|          |              |              |`SPI4_PCS0`|              |                  |              |         |         |         |
//! |  D11  |`SD_B0_02`|          |              |              |`SPI4_SDO` |              |                  |              |         |         |         |
//! |  D12  |`SD_B0_03`|          |              |              |`SPI4_SDI` |              |                  |              |         |         |         |
//! |  D13  |`SD_B0_00`|          |              |              |`SPI4_SCK` |              |`GPIO2_3`         |              |         |         |         |
//! |  D14  |`AD_B1_01`|          |              |  `UART2_TX`  |           |              |                  |              |         |         |         |
//! |  D15  |`AD_B1_00`|          |              |  `UART2_RX`  |           |              |                  |              |         |         |         |
//! |  A0   |`AD_B1_10`|          |`I2C3_SCL`    |  `UART3_RX`  |           |              |                  |              |         |         |         |
//! |  A1   |`AD_B1_11`|          |`I2C3_SDA`    |  `UART3_TX`  |           |              |                  |              |         |         |         |
//! |  A2   |`AD_B1_04`|          |              |              |`I2C1_SDA` |              |                  |              |         |         |         |
//! |  A3   |`AD_B1_05`|          |              |  `UART2_CTS` |`I2C1_SCL` |              |                  |              |         |         |         |
//! |  A4   |`AD_B1_01`|          |              |  `UART8_TX`  |           |              |                  |              |         |         |         |
//! |  A5   |`AD_B1_00`|          |              |  `UART8_TX`  |           |              |                  |              |         |         |         |
//!
//!
//! ## User LED
//! GPIO_AD_B0_09
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

// Needed generated boot configuration data
include!(concat!(env!("OUT_DIR"), "/fcb.rs"));

pub use hal::ral::interrupt;
pub use imxrt_rt  as rt;
pub use imxrt_ral as ral;
pub use imxrt_hal as hal;

pub type LED = hal::gpio::GPIO1IO09<hal::gpio::GPIO1, hal::gpio::Output>;

pub use hal::ccm::CCM;

/// Arduino pins that do not yet have a function
///
/// Pin 13 can be used for several things; one common usage is for the on-board LED.
pub struct Pins {
    pub d0: hal::iomuxc::gpio::GPIO_AD_B1_07<hal::iomuxc::Alt5>,
    pub d1: hal::iomuxc::gpio::GPIO_AD_B1_06<hal::iomuxc::Alt5>,
    pub d2: hal::iomuxc::gpio::GPIO_AD_B0_11<hal::iomuxc::Alt0>,
    pub d3: hal::iomuxc::gpio::GPIO_AD_B1_08<hal::iomuxc::Alt5>,
    pub d4: hal::iomuxc::gpio::GPIO_AD_B0_09<hal::iomuxc::Alt0>,
    pub d5: hal::iomuxc::gpio::GPIO_AD_B0_10<hal::iomuxc::Alt0>,
    pub d6: hal::iomuxc::gpio::GPIO_AD_B1_02<hal::iomuxc::Alt5>,
    pub d7: hal::iomuxc::gpio::GPIO_AD_B1_03<hal::iomuxc::Alt5>,
    pub d8: hal::iomuxc::gpio::GPIO_AD_B0_03<hal::iomuxc::Alt5>,
    pub d9: hal::iomuxc::gpio::GPIO_AD_B0_02<hal::iomuxc::Alt5>,
    pub d10: hal::iomuxc::gpio::GPIO_SD_B0_01<hal::iomuxc::Alt5>,
    pub d11: hal::iomuxc::gpio::GPIO_SD_B0_02<hal::iomuxc::Alt5>,
    pub d12: hal::iomuxc::gpio::GPIO_SD_B0_03<hal::iomuxc::Alt5>,
    pub d13: hal::iomuxc::gpio::GPIO_SD_B0_00<hal::iomuxc::Alt5>,
    pub d14: hal::iomuxc::gpio::GPIO_AD_B1_01<hal::iomuxc::Alt5>,
    pub d15: hal::iomuxc::gpio::GPIO_AD_B1_00<hal::iomuxc::Alt5>,
    pub a0: hal::iomuxc::gpio::GPIO_AD_B1_10<hal::iomuxc::Alt5>,
    pub a1: hal::iomuxc::gpio::GPIO_AD_B1_11<hal::iomuxc::Alt5>,
    pub a2: hal::iomuxc::gpio::GPIO_AD_B1_04<hal::iomuxc::Alt5>,
    pub a3: hal::iomuxc::gpio::GPIO_AD_B1_05<hal::iomuxc::Alt5>,
}

/// All peripherals available on the Teensy4
pub struct Peripherals {
    /// Clock control module (forwarded from the HAL)
    pub ccm: hal::ccm::CCM,
    /// PIT timers (forwarded from the HAL)
    pub pit: hal::pit::UnclockedPIT,
    /// DCDC converters
    pub dcdc: hal::dcdc::DCDC,
    /// PWM1 controller
    pub pwm1: hal::pwm::Unclocked<hal::pwm::module::_1>,
    /// PWM2 controller
    pub pwm2: hal::pwm::Unclocked<hal::pwm::module::_2>,
    /// PWM3 controller
    pub pwm3: hal::pwm::Unclocked<hal::pwm::module::_3>,
    /// PWM4 controller
    pub pwm4: hal::pwm::Unclocked<hal::pwm::module::_4>,
    /// Pins
    pub pins: Pins,
    /// Unclocked I2C peripherals
    pub i2c: hal::i2c::Unclocked,
    /// Unclocked SPI peripherals
    pub spi: hal::spi::Unclocked,
    /// Unclocked UART peripherals
    pub uart: hal::uart::Unclocked,
    /// General purpose registers, used when configuring GPIO pins.
    pub gpr: hal::iomuxc::GPR,
}

/// SYSTICK external clock frequency
///
/// See Section 12.3.2.1 of the reference manual. The note
/// explains that the 24MHz clock is divided down to 100KHz
/// before reaching SYSTICK.
const SYSTICK_EXT_FREQ: u32 = 100_000;

impl Peripherals {
    /// Instantiate the system peripherals. This may only be called once!
    pub fn take() -> Option<Self> {
        let p = hal::Peripherals::take()?;
        let mut cp = cortex_m::Peripherals::take()?;
        //Self::set_systick(&mut cp.SYST);
        Some(Peripherals::new(p))
    }

    fn set_systick(systick: &mut cortex_m::peripheral::SYST) {
        systick.disable_counter();
        systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
        systick.set_reload((SYSTICK_EXT_FREQ / 1000) - 1);
        systick.clear_current();
        systick.enable_counter();
        systick.enable_interrupt();
    }

    fn new(p: hal::Peripherals) -> Peripherals {
        Peripherals {
            ccm: p.ccm,
            pit: p.pit,
            dcdc: p.dcdc,
            pwm1: p.pwm1,
            pwm2: p.pwm2,
            pwm3: p.pwm3,
            pwm4: p.pwm4,
            pins: Pins {
                d0: p.iomuxc.gpio_ad_b1_07,
                d1: p.iomuxc.gpio_ad_b1_06,
                d2: p.iomuxc.gpio_ad_b0_11,
                d3: p.iomuxc.gpio_ad_b1_08,
                d4: p.iomuxc.gpio_ad_b0_09,
                d5: p.iomuxc.gpio_ad_b0_10,
                d6: p.iomuxc.gpio_ad_b1_02,
                d7: p.iomuxc.gpio_ad_b1_03,
                d8: p.iomuxc.gpio_ad_b0_03,
                d9: p.iomuxc.gpio_ad_b0_02,
                d10: p.iomuxc.gpio_sd_b0_01,
                d11: p.iomuxc.gpio_sd_b0_02,
                d12: p.iomuxc.gpio_sd_b0_03,
                d13: p.iomuxc.gpio_sd_b0_00,
                d14: p.iomuxc.gpio_ad_b1_01,
                d15: p.iomuxc.gpio_ad_b1_00,
                a0: p.iomuxc.gpio_ad_b1_10,
                a1: p.iomuxc.gpio_ad_b1_11,
                a2: p.iomuxc.gpio_ad_b1_04,
                a3: p.iomuxc.gpio_ad_b1_05,
            },
            i2c: p.i2c,
            spi: p.spi,
            uart: p.uart,
            gpr: p.iomuxc.gpr,
        }
    }
}

pub fn configure_led(
    gpr: &mut hal::iomuxc::GPR,
    pad: hal::iomuxc::gpio::GPIO_AD_B0_09<hal::iomuxc::Alt5>,
) -> LED {
    use ral::modify_reg;
    use hal::gpio::IntoGpio;
    let pin = pad.into_gpio().output();
    unsafe {
        modify_reg!(ral::iomuxc, IOMUXC, SW_PAD_CTL_PAD_GPIO_AD_B0_09,
                    PKE: PKE_1_Pull_Keeper_Enabled,
                    SPEED: SPEED_2_medium_100MHz,
                    DSE: DSE_6_R0_6
        );
    };
    pin
}

/// Blocks for at least `millis` milliseconds
///
/// `delay()` will spin-loop on updates from SYSTICK, until
/// `millis` milliseconds have elapsed. SYSTICK has a 1ms
/// interrupt interval, so the minimal delay is around 1ms.
#[no_mangle]
pub extern "C" fn delay(millis: u32) {
    if 0 == millis {
        return;
    }
    let start = systick::read();
    let target = start + millis;
    loop {
        let count = systick::read();
        if count >= target {
            return;
        }
    }
}

/// Scoping of data related to SYSTICK
mod systick {
    use imxrt_rt::exception;

    #[no_mangle]
    static mut systick_millis_count: u32 = 0;

    #[exception]
    fn SysTick() {
        unsafe {
            let ms = core::ptr::read_volatile(&systick_millis_count);
            let ms = ms.wrapping_add(1);
            core::ptr::write_volatile(&mut systick_millis_count, ms);
        }
    }

    /// Read the systick counter. Returns an absolute value describing
    /// the number of milliseconds since the SYSTICK handler was enabled.
    /// This may be used to implement coarse timing.
    pub fn read() -> u32 {
        unsafe { core::ptr::read_volatile(&systick_millis_count) }
    }
}
