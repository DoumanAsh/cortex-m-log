//! Contains possible printers for Cortex-M

use core::fmt;
use core::fmt::Write;
use crate::modes::{InterruptModer};

///Generic Printer trait
pub trait Printer {
    ///Writer type
    type W: fmt::Write;
    ///Interrupt type
    type M: InterruptModer;

    ///Returns destination writer.
    ///
    ///Used by `print` and `println` default
    ///impls
    fn destination(&mut self) -> &mut Self::W;

    #[inline]
    ///Prints formatted output to destination
    fn print(&mut self, args: fmt::Arguments) {
        let _ = Self::M::critical_section(|_| self.destination().write_fmt(args));
    }

    #[inline]
    ///Prints formatted output to destination with newline
    fn println(&mut self, args: fmt::Arguments) {
        let _ = Self::M::critical_section(|_| {
            let _ = self.destination().write_fmt(args);
            self.destination().write_str("\n")
        });
    }
}

pub mod generic;
pub use generic::GenericPrinter;

pub mod dummy;
pub use self::dummy::Dummy;

#[cfg(feature = "itm")]
pub mod itm;
#[cfg(feature = "itm")]
pub use self::itm::Itm;

#[cfg(feature = "semihosting")]
pub mod semihosting;
#[cfg(feature = "semihosting")]
pub use self::semihosting::{Semihosting};
