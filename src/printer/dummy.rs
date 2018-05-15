//! Dummy module

use core::fmt;

/// Dummy printer
pub struct Dummy;

impl super::Printer for Dummy {
    type W = ::destination::Dummy;
    type M = ::modes::InterruptFree;

    #[inline]
    fn destination(&mut self) -> &mut Self::W {
        unreachable!()
    }

    #[inline]
    fn print(&mut self, _: fmt::Arguments) {}

    #[inline]
    fn println(&mut self, _: fmt::Arguments) {}
}
