//! Semihosting module

extern crate cortex_m_semihosting as sh;

use core::fmt;
use core::fmt::Write;

/// Trait implemented for writes that can be used with [SH](struct.SH.html)
pub trait SemihostingComp: Write {}

impl SemihostingComp for sh::hio::HostStream {}

/// Semihosting destination
pub struct SH<T: SemihostingComp> {
    inner: T,
}

impl<T: SemihostingComp> SH<T> {
    #[inline]
    ///Creates new instance
    pub fn new(inner: T) -> SH<T> {
        Self {
            inner
        }
    }
}

impl<T: SemihostingComp> Write for SH<T> {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.inner.write_str(s)
    }
}

/// Alias to semihosting's stdout
pub type SHStream = SH<sh::hio::HostStream>;
