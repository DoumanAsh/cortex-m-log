//! ITM module

extern crate cortex_m;

use self::cortex_m::{itm};
use core::fmt;

/// ITM based destination
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
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let stim = &mut self.inner.stim[0];
        itm::write_str(stim, s);

        Ok(())
    }

    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        let stim = &mut self.inner.stim[0];
        itm::write_fmt(stim, args);

        Ok(())
    }
}
