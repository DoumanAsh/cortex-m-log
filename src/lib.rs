//! Logging facilities for Cortex-M processors
//!
//! All writes are interrupt free
#![no_std]

extern crate cortex_m;

#[cfg(feature = "dummy")]
pub mod dummy;
#[cfg(feature = "dummy")]
pub use dummy::Dummy;

#[cfg(feature = "itm")]
pub mod itm;
#[cfg(feature = "itm")]
pub use itm::Itm;

#[cfg(feature = "semihosting")]
pub mod semihosting;
#[cfg(feature = "semihosting")]
pub use semihosting::{SHerr, SHout};
