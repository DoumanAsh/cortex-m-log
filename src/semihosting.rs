//! Semihosting module

extern crate cortex_m;
extern crate cortex_m_semihosting as sh;

use self::cortex_m::interrupt;
use core::fmt;
use core::fmt::Write;

/// Trait implemented for writes that can be used with [SH](struct.SH.html)
pub trait SemihostingComp: Write {}

impl SemihostingComp for sh::hio::HStdout {}
impl SemihostingComp for sh::hio::HStderr {}

/// Semihosting logger
pub struct SH<T: SemihostingComp> {
    inner: T,
}

impl<T: SemihostingComp> SH<T> {
    pub fn new(typ: T) -> SH<T> {
        Self { inner: typ }
    }
}

impl<T: SemihostingComp> Write for SH<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        interrupt::free(|_| self.inner.write_str(s))
    }
}

/// Alias to semihosting's stdout
pub type SHout = SH<sh::hio::HStdout>;
/// Alias to semihosting's stderr
pub type SHerr = SH<sh::hio::HStderr>;
