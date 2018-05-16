//! Dummy module

use core::fmt;

/// Dummy destination
pub struct Dummy;

impl fmt::Write for Dummy {
    #[inline]
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, _: char) -> fmt::Result {
        Ok(())
    }

    #[inline]
    fn write_fmt(&mut self, _: fmt::Arguments) -> fmt::Result {
        Ok(())
    }
}
