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
//! ## Macros
//!
//! The crate provide primitieve macros that are enabled only in debug release.
//! Controlled by `debug_assertions` attribute
//!
//! ```rust
//! #[macro_use(print, println, d_print)]
//! extern crate cortex_m_log;
//! extern crate log;
//!
//! use cortex_m_log::printer::Dummy;
//!
//! fn main() {
//!     let mut log = Dummy;
//!     println!(log, "Some print with newline!");
//!     //Debug version of print that resolves into nothing in release mode
//!     //Note that you must import print macro for it to work
//!     d_print!(log, "Print stuff: {}", "stuff");
//!     //Note that you must import println macro for it to work
//!     d_print!(log, "Print stuff: {} and also newline", "stuff");
//! }
//! ```

#![no_std]

extern crate cortex_m;

pub mod modes;
pub mod destination;
pub mod printer;
#[cfg(feature = "log-integration")]
pub mod log;

///Print macro that uses Printers to write formatted output
#[macro_export]
macro_rules! print {
    ($logger:expr, $($arg:tt)+) => ({
        use $crate::printer::Printer;
        $logger.print(format_args!($($arg)+));
    })
}

///Print macro that uses Printers to write formatted output with newline
#[macro_export]
macro_rules! println {
    ($logger:expr, $($arg:tt)+) => ({
        use $crate::printer::Printer;
        $logger.println(format_args!($($arg)+));
    })
}

///Deubg print macro that uses Printers to write formatted output
///
///It is defined to empty macro unless `debug_assertions` is enabled
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! d_print {
    ($logger:expr, $($arg:tt)+) => ({
    })
}

///Deubg print macro that uses Printers to write formatted output
///
///It is defined to empty macro unless `debug_assertions` is enabled
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! d_print {
    ($logger:expr, $($arg:tt)+) => ({
        print!($logger, $($arg)+)
    })
}

///Deubg print macro that uses Printers to write formatted output with newline
///
///It is defined to empty macro unless `debug_assertions` is enabled
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! d_println {
    ($logger:expr, $($arg:tt)+) => ({
    })
}

///Deubg print macro that uses Printers to write formatted output with newline
///
///It is defined to empty macro unless `debug_assertions` is enabled
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! d_println {
    ($logger:expr, $($arg:tt)+) => ({
        println!($logger, $($arg)+)
    })
}
