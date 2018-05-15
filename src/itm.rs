//! ITM module

extern crate aligned;
extern crate cortex_m;

use self::cortex_m::{interrupt, itm};
use core::fmt;

/// ITM based logger
pub struct Itm {
    inner: cortex_m::peripheral::ITM,
}

impl Itm {
    /// Creates new instance by taking ownership of ITM register block
    pub fn new(itm: cortex_m::peripheral::ITM) -> Self {
        Self { inner: itm }
    }
}

impl fmt::Write for Itm {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let stim = &mut self.inner.stim[0];
        interrupt::free(|_| {
            itm::write_str(stim, s);
        });

        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        let stim = &mut self.inner.stim[0];
        interrupt::free(|_| {
            itm::write_fmt(stim, args);
        });

        Ok(())
    }
}
