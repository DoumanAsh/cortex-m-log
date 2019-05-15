//! Dummy module

use core::fmt;

/// Dummy printer
pub struct Dummy {
    w: crate::destination::Dummy,
}

impl Dummy {
    /// Create dummy printer
    pub fn new() -> Self {
        Dummy { w: crate::destination::Dummy }
    }
}

impl super::Printer for Dummy {
    type W = crate::destination::Dummy;
    type M = crate::modes::InterruptFree;

    #[inline]
    fn destination(&mut self) -> &mut Self::W {
        &mut self.w
    }

    #[inline]
    fn print(&mut self, _: fmt::Arguments) {}

    #[inline]
    fn println(&mut self, _: fmt::Arguments) {}
}
