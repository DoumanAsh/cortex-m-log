//! Dummy module

use core::fmt;

/// Dummy logger
pub struct Dummy;

impl fmt::Write for Dummy {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Ok(())
    }
}
