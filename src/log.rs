//! Integration with [log](https://docs.rs/crate/log)
extern crate log;
extern crate cortex_m_semihosting as sh;

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
