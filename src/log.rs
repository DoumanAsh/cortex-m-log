//! Integration with [log](https://docs.rs/crate/log)
//!
//! As embedded development implies `#[no_st]` we cannot box
//! our logger so instead simplest approach would be to declare our logger static
//!
//! ```rust
//! extern crate cortex_m_log;
//! extern crate log;
//!
//! use cortex_m_log::log::{Logger, init};
//! use cortex_m_log::printer::Dummy;
//!
//! static LOGGER: Logger<Dummy> = Logger {
//!     inner: Dummy,
//!     level: log::LevelFilter::Off
//! };
//!
//! fn main() {
//!     init(&LOGGER).expect("To set logger");
//! }
//! ```
//!
//! Of course since there is no `const-fn` and some printers require
//! Initialization function we need some better way.
//!
//! One way to do it would be to trick compiler by changing lifetime of stack
//! allocated logger to static, obviously unsafe.
//!
//! ```rust,no_run
//! extern crate cortex_m_log;
//! extern crate log;
//! extern crate cortex_m_semihosting;
//!
//! use std::mem;
//!
//! use cortex_m_log::log::{Logger, trick_init};
//! use cortex_m_log::printer::semihosting;
//! use cortex_m_log::modes::InterruptOk;
//!
//! fn main() {
//!     let logger = Logger {
//!         inner: semihosting::InterruptOk::<_>::stdout().expect("Get Semihosting stdout"),
//!         level: log::LevelFilter::Info
//!     };
//!     //Haha trust me, it is safe ;)
//!     //As long as logger is not dropped....
//!     unsafe {
//!         let _ = trick_init(&logger);
//!     }
//! }
//! ```
//!
//! Obviously it is UB to drop logger after that and use any of log's macros
extern crate log;
extern crate cortex_m_semihosting as sh;

use core::mem;
use core::marker;
use ::printer::Printer;

///Simple Logger implementation
pub struct Logger<P: Printer + marker::Send + marker::Sync> {
    ///Printer implementation
    pub inner: P,
    ///Log level
    pub level: log::LevelFilter,
}

impl<P: Printer + marker::Send + marker::Sync> log::Log for Logger<P> {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            //Pretend we're the Cell
            let inner = &self.inner as *const P as *mut P;
            let inner = unsafe { &mut *inner };

            inner.print(format_args!("{:<5} {}:{} - {}\n", record.level(), record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0), record.args()))
        }
    }

    fn flush(&self) {
    }
}

///Initialize logging facilities.
///
///Currently lacks a nice way to set global logger so user
///must himself guarantee static lifetime
pub fn init<P: Printer + marker::Send + marker::Sync>(logger: &'static Logger<P>) -> Result<(), log::SetLoggerError> {
    log::set_max_level(logger.level.clone());
    log::set_logger(logger)
}

#[inline]
///Performs init by tricking compiler into beliving that `&mut` is static.
///
///This is unsafe and it is up to you to ensure that reference
///will live longer that any attempts to access it
pub unsafe fn trick_init<P: Printer + marker::Send + marker::Sync + 'static>(logger: &Logger<P>) -> Result<(), log::SetLoggerError> {
    let logger: &'static Logger<P> = mem::transmute(logger);
    init(logger)
}
