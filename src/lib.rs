//! Logging facilities for Cortex-M processors
//!
//! ## Destinations
//!
//! The crate provides following destinations for writes:
//! - [Dummy](destination/dummy/struct.Dummy.html) - noop destination that performs no writes. Useful for release mode
//! - [Itm](destination/itm/struct.Itm.html) - Uses Cortex-M [Itm](https://docs.rs/cortex-m/0.5.1/cortex_m/itm/index.html) to send output. Note that it is available only on ARMv7-M and newer
//! - [Semihosting](destination/semihosting/struct.SemihostingComp.html) - Uses Cortex-M [Semihosting](https://docs.rs/cortex-m-semihosting) to send output.
//!
//! All destinations implements [fmt::Write](https://doc.rust-lang.org/core/fmt/trait.Write.html)
//! to provide simple and generic interface
//!
//! ## Printers
//!
//! Each destination is provided with corresponding [Printer](printer/trait.Printer.html).
//! In addition to providing generic interface it also allows to configure Interrupt mode for all
//! prints. [See](modes/index.html).
//!
#![no_std]

extern crate cortex_m;

pub mod modes;
pub mod destination;
pub mod printer;
#[cfg(feature = "log-integration")]
pub mod log;
