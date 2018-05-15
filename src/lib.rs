//! Logging facilities for Cortex-M processors
//!
//! All writes are interrupt free
#![no_std]

extern crate cortex_m;

pub mod modes;
pub mod destination;
pub mod printer;

