//! Generic printer

use core::fmt;

///Generic printer which works with any `core::fmt::Writer` compatible type
pub struct GenericPrinter<W> {
    writer: W,
}

impl<W> GenericPrinter<W> {
    /// Create dummy printer
    pub const fn new(writer: W) -> Self {
        Self {
            writer
        }
    }
}

impl<W: fmt::Write> super::Printer for GenericPrinter<W> {
    type W = W;
    type M = crate::modes::InterruptFree;

    #[inline]
    fn destination(&mut self) -> &mut Self::W {
        &mut self.writer
    }
}
