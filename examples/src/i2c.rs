//! Demonstrates an I2C master. We try to read data from
//! a BME280 Pressure, Humidity, Temperature Sensor with SDO
//! tied to ground giving the device its 0x76 byte address.
//!
//! Pin d15 => SCL (I2C1)
//! Pin d14 => SDA (I2C1)
//!
//! Success criteria:
//!
//! - The Sensor correctly reports its `WHO_AM_I` address. The slave
//!   address is printed over UART logging.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::i2c::ClockSpeed;
use embedded_hal::blocking::i2c;
use embedded_hal::digital::v2::OutputPin;
use cortex_m::asm::nop;
use imxrt1060evk_bsp as bsp;

/// MPU9250 I2C slave address
const SLAVE_ADDRESS: u8 = 0x68;
/// Our I2C clock speed. Change me to try 400KHz
const I2C_CLOCK_SPEED: ClockSpeed = ClockSpeed::KHz400;

/// Returns the MPU's WHO_AM_I value. This should be a static
/// value that's specific for a MPU variant.
fn who_am_i<I>(i2c: &mut I) -> Result<u8, I::Error>
where
    I: i2c::WriteRead,
{
    const WHO_AM_I: u8 = 0xD0;
    let mut out = [0; 1];
    i2c.write_read(SLAVE_ADDRESS, &[WHO_AM_I], &mut out)?;
    Ok(out[0])
}

#[bsp::rt::entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut led = bsp::configure_led(&mut peripherals.gpr, peripherals.pins.d4.alt5());

    let (i2c1_builder, _, _, _) = peripherals.i2c.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::i2c::ClockSelect::OSC, // 2MHz clock...
        bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_3, // Divide by 3
    );

    //log::info!("Constructing I2C1 instance on pins d14 and d15...");
    let mut i2c1 = i2c1_builder.build(peripherals.pins.d15.alt3(), peripherals.pins.d14.alt3());

    if let Err(err) = i2c1.set_bus_idle_timeout(core::time::Duration::from_micros(200)) {
        //log::warn!("Error when setting bus idle timeout: {:?}", err);
        cortex_m::asm::nop();
    }
    if let Err(err) = i2c1.set_pin_low_timeout(core::time::Duration::from_millis(1)) {
        //log::warn!("Error when setting pin low timeout: {:?}", err);
        cortex_m::asm::nop();
    }
    if let Err(err) = i2c1.set_clock_speed(I2C_CLOCK_SPEED) {
        //log::warn!(
        //    "Error when setting I2C clock speed to {:?}: {:?}",
        //    I2C_CLOCK_SPEED,
        //    err
        //);
        cortex_m::asm::nop();
    }

    //log::info!("Starting I/O loop...");
    loop {
        bsp::delay(1000);
        //log::info!("Querying WHO_AM_I...");

        match who_am_i(&mut i2c1) {
            Ok(who) => 
                led.set_low().unwrap(),
                //log::info!("Received 0x{:X} for WHO_AM_I", who),
            Err(err) => {
                //log::warn!("Error reading WHO_AM_I: {:?}", err);
                continue;
            }
        }
    }
}
