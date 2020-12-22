//! Hardware pins for NXP's imxrt1060evk board
//!
//! `imxrt1060evk-pins` provides types and a struct containing the set of Pins
//! the board exposes to other devices and IO connectors.
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
//! |  D0   |`AD_B1_07`|          |              |              |           |              |                  |              |         |         |         |
//! |  D1   |`AD_B1_06`|          |              |              |           |              |                  |              |         |         |         |
//! |  D2   |`AD_B0_11`|          |`FlexPWM1_3_B`|              |           |              |                  |              |         |         |         |
//! |  D3   |`AD_B1_08`|          |              |              |           |              |                  |              |         |         |         |
//! |  D4   |`AD_B0_09`|          |              |              |           |              |                  |              |         |         |         |
//! |  D5   |`AD_B0_10`|          |`FlexPWM1_3_A`|              |           |              |                  |              |         |         |         |
//! |  D6   |`AD_B1_02`|          |              |              |           |              |                  |              |         |         |         |
//! |  D7   |`AD_B1_03`|          |              |              |           |              |                  |              |         |         |         |
//! |  D8   |`AD_B0_03`|          |              |              |           |              |                  |              |         |         |         |
//! |  D9   |`AD_B0_02`|          |              |              |           |              |                  |              |         |         |         |
//! |  D10  |`SD_B0_01`|          |`FlexPWM1_0_B`|              |           |              |                  |              |         |         |         |
//! |  D11  |`SD_B0_02`|          |              |              |           |              |                  |              |         |         |         |
//! |  D12  |`SD_B0_03`|          |              |              |           |              |                  |              |         |         |         |
//! |  D13  |`SD_B0_00`|          |`FlexPWM1_0_A`|              |           |              |`GPIO2_3`         |              |         |         |         |
//! |  D14  |`AD_B1_01`|          |              |              |           |              |                  |              |         |         |         |
//! |  D15  |`AD_B1_00`|          |              |              |           |              |                  |              |         |         |         |
//! |  A0   |`AD_B1_10`|          |              |              |           |              |                  |              |         |         |         |
//! |  A1   |`AD_B1_11`|          |              |              |           |              |                  |              |         |         |         |
//! |  A2   |`AD_B1_04`|          |              |              |           |              |                  |              |         |         |         |
//! |  A3   |`AD_B1_05`|          |              |              |           |              |                  |              |         |         |         |
//! |  A4   |`AD_B1_01`|          |              |              |           |              |                  |              |         |         |         |
//! |  A5   |`AD_B1_00`|          |              |              |           |              |                  |              |         |         |         |
//!
//!
//! ## UART1 - connected to OpenSDA debugger
//! | Pin      | Pad ID   |  Alt2        |
//! | -----    | -------- | ------------ |
//! |  UART_TX |`AD_B0_12`|  `UART1_TX`  |
//! |  UART_RX |`AD_B0_13`|  `UART1_RX`  |
//!
//! ## LED - Connected to Arduino Header and on board LED
//! | Pin   | Pad ID   | Alt5  |
//! | ----- | -------- | ----- |
//! | D4    |`AD_B0_09`|`GPIO` |
//!
//! Not every pin is connected to the SoC.

mod iomuxc {
    pub use imxrt_iomuxc::imxrt106x::*;
    pub use imxrt_iomuxc::prelude::*;
}

use crate::iomuxc::{ad_b0::*, ad_b1::*, sd_b0::*, ErasedPad};

/// Arduino D0
pub type D0 = AD_B1_07;
/// Arduino D1
pub type D1 = AD_B1_06;
/// Arduino D2
pub type D2 = AD_B0_11;
/// Arduino D3
pub type D3 = AD_B1_08;
/// Arduino D4
pub type D4 = AD_B0_09;
/// Arduino D5
pub type D5 = AD_B0_10;
/// Arduino D6
pub type D6 = AD_B1_02;
/// Arduino D7
pub type D7 = AD_B1_03;
/// Arduino D8
pub type D8 = AD_B0_03;
/// Arduino D9
pub type D9 = AD_B0_02;
/// Arduino D10
pub type D10 = SD_B0_01;
/// Arduino D11
pub type D11 = SD_B0_02;
/// Arduino D12
pub type D12 = SD_B0_03;
/// Arduino D13
pub type D13 = SD_B0_00;
/// Arduino D14
pub type D14 = AD_B1_01;
/// Arduino D15
pub type D15 = AD_B1_00;
/// Arduino A0
pub type A0 = AD_B1_10;
/// Arduino A1
pub type A1 = AD_B1_11;
/// Arduino A2
pub type A2 = AD_B1_04;
/// Arduino A3
pub type A3 = AD_B1_05;

/// UartTx pin connected to onboard LPC4322 DAP-LINK
type UartTx = AD_B0_12;
/// UartRx pin connected to onboard LPC4322 DAP-LINK
type UartRx = AD_B0_13;

/// Type-erased pins
///
///
/// Use [`Pins::erase`](struct.Pins.html#method.erase) to erase pin types.
pub type ErasedPins = [ErasedPad; 20];

/// MIMXRT1060-EVK pins
///
/// See [`into_pins`](fn.into_pins.html) to safely constrain the processor's pads, and acquire
/// pins. Or, use [`new`](#method.new) to unsafely create pins.
pub struct Pins {
    /// Arduino D0
    pub d0: D0,
    /// Arduino D1
    pub d1: D1,
    /// Arduino D2
    pub d2: D2,
    /// Arduino D3
    pub d3: D3,
    /// Arduino D4
    pub d4: D4,
    /// Arduino D5
    pub d5: D5,
    /// Arduino D6
    pub d6: D6,
    /// Arduino D7
    pub d7: D7,
    /// Arduino D8
    pub d8: D8,
    /// Arduino D9
    pub d9: D9,
    /// Arduino D10
    pub d10: D10,
    /// Arduino D11
    pub d11: D11,
    /// Arduino D12
    pub d12: D12,
    /// Arduino D13
    pub d13: D13,
    /// Arduino D14
    pub d14: D14,
    /// Arduino D15
    pub d15: D15,
    /// Arduino A0
    pub a0: A0,
    /// Arduino A1
    pub a1: A1,
    /// Arduino A2
    pub a2: A2,
    /// Arduino A3
    pub a3: A3,
    // Built-in debug probe uart tx
    pub uart_tx: UartTx,
    // Built-in debug probe uart rx
    pub uart_rx: UartRx,
}

/// Constrain the processor pads to the Teensy 4.0 pins
pub const fn into_pins(iomuxc: crate::iomuxc::Pads) -> Pins {
    Pins {
        d0: iomuxc.ad_b1.p07,
        d1: iomuxc.ad_b1.p06,
        d2: iomuxc.ad_b0.p11,
        d3: iomuxc.ad_b1.p08,
        d4: iomuxc.ad_b0.p09,
        d5: iomuxc.ad_b0.p10,
        d6: iomuxc.ad_b1.p02,
        d7: iomuxc.ad_b1.p03,
        d8: iomuxc.ad_b0.p03,
        d9: iomuxc.ad_b0.p02,
        d10: iomuxc.sd_b0.p01,
        d11: iomuxc.sd_b0.p02,
        d12: iomuxc.sd_b0.p03,
        d13: iomuxc.sd_b0.p00,
        d14: iomuxc.ad_b1.p01,
        d15: iomuxc.ad_b1.p00,
        a0: iomuxc.ad_b1.p10,
        a1: iomuxc.ad_b1.p11,
        a2: iomuxc.ad_b1.p04,
        a3: iomuxc.ad_b1.p05,
        uart_tx: iomuxc.ad_b0.p12,
        uart_rx: iomuxc.ad_b0.p13,
    }
}

impl Pins {
    /// Create an instance of `Pins` when you do not have a handle
    /// to the processor pads
    ///
    /// # Safety
    ///
    /// Caller must ensure that the pins are not aliased elsewhere in
    /// the program. This could include
    ///
    /// - an existing handle to the `imxrt-iomuxc` pads,
    /// - another instance of `Pins` that was safely acquired
    ///   using [`into_pins`](fn.into_pins.html).
    pub const unsafe fn new() -> Self {
        into_pins(crate::iomuxc::Pads::new())
    }

    /// Erase the types of all pins
    pub fn erase(self) -> ErasedPins {
        [
            self.d0.erase(),
            self.d1.erase(),
            self.d2.erase(),
            self.d3.erase(),
            self.d4.erase(),
            self.d5.erase(),
            self.d6.erase(),
            self.d7.erase(),
            self.d8.erase(),
            self.d9.erase(),
            self.d10.erase(),
            self.d11.erase(),
            self.d12.erase(),
            self.d13.erase(),
            self.d14.erase(),
            self.d15.erase(),
            self.a0.erase(),
            self.a1.erase(),
            self.a2.erase(),
            self.a3.erase(),
        ]
    }
}
