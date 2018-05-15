//! Dummy module

use core::fmt;

/// Dummy destination
pub struct Dummy;

impl fmt::Write for Dummy {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Ok(())
    }

    fn write_char(&mut self, _: char) -> fmt::Result {
        Ok(())
    }

    fn write_fmt(&mut self, _: fmt::Arguments) -> fmt::Result {
        Ok(())
    }
}
