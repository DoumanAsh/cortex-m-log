//! ITM module

use cortex_m::{itm, interrupt::Mutex};
use core::fmt;

use core::cell::RefCell;

/// ITM based destination
pub struct Itm {
    inner: Mutex<RefCell<cortex_m::peripheral::ITM>>,
}

impl Itm {
    /// Creates new instance by taking ownership of ITM register block
    pub fn new(itm: cortex_m::peripheral::ITM) -> Self {
        Self { inner: Mutex::new(RefCell::new(itm)) }
    }
}

impl fmt::Write for Itm {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        cortex_m::interrupt::free(|cs| {
            if let Some(mut itm) = self.inner.borrow(cs).try_borrow_mut().ok() {
                let stim = &mut itm.stim[0];
                itm::write_str(stim, s);
            }
        });
        Ok(())
    }

    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        cortex_m::interrupt::free(|cs| {
            if let Some(mut itm) = self.inner.borrow(cs).try_borrow_mut().ok() {
                let stim = &mut itm.stim[0];
                itm::write_fmt(stim, args);
            }
        });

        Ok(())
    }
}
