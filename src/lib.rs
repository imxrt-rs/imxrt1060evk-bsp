//! A Rust board support package (BSP) for the IMXRT1060EVK
//!
//! As of this writing, the BSP is very primitive. It exposes
//! only the LED.
//!
//! ## Re-exports
//!
//! The BSP re-exports the following:
//!
//! - the `imxrt-rt` crate, as `rt`
//! - the `imxrt-ral` crate, as `ral`
//!
//! See the accompanying documentation of each crate for more
//! information.
//!
//! For simplicity, there may be other choice APIs from either crate that
//! are re-exported in the BSP namespace.
//!
//! Very much in a state of heavy development.

#![no_std]

// Needed generated boot configuration data
include!(concat!(env!("OUT_DIR"), "/fcb.rs"));

pub use imxrt_ral as ral;
//pub use imxrt_hal as hal;
pub use imxrt_rt as rt;
