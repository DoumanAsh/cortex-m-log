//! Semihosting module
//!
//! Semihosting can direct to either stdout or stderr,
//! which is determined by second type parameter
//!
//! ## Example
//!
//! ```rust,no_run
//! use cortex_m_log::printer::Printer;
//! use cortex_m_log::printer::semihosting;
//! use cortex_m_log::printer::semihosting::Semihosting;
//! use cortex_m_log::modes::InterruptFree;
//!
//! //Create new printer to stdout with interrupt free mode
//! let mut shost = Semihosting::<InterruptFree, _>::stdout().unwrap();
//!
//! shost.println(format_args!("Write interrupt free to {}", "stdout"));
//!
//! //Create new printer to stderr without need for interrupt control using special alias
//! let mut shost = semihosting::InterruptOk::<_>::stderr().unwrap();
//!
//! shost.println(format_args!("Write to {}", "stderr"));
//! ```

extern crate cortex_m_semihosting as sh;

pub use self::sh::hio;

use crate::destination::semihosting::SemihostingComp;
use crate::modes::InterruptModer;

use core::marker::PhantomData;

/// Semihosting backed printer
pub struct Semihosting<M: InterruptModer, T: SemihostingComp> {
    inner: T,
    _mod: PhantomData<M>
}

impl<M: InterruptModer, T: SemihostingComp> Semihosting<M, T> {
    ///Constructs new instance
    pub fn new(inner: T) -> Self {
        Self { inner, _mod: PhantomData }
    }
}

impl<M: InterruptModer> Semihosting<M, hio::HStdout> {
    ///Constructs new Semihosting Printer by using `HStdout`
    pub fn stdout() -> Result<Self, ()> {
        hio::hstdout().map(|stdout| Self::new(stdout))
    }
}

impl<M: InterruptModer> Semihosting<M, hio::HStderr> {
    ///Constructs new Semihosting Printer by using `HStderr`
    pub fn stderr() -> Result<Self, ()> {
        hio::hstderr().map(|stderr| Self::new(stderr))
    }
}

impl<Mode: InterruptModer, T: SemihostingComp> super::Printer for Semihosting<Mode, T> {
    type W = T;
    type M = Mode;

    #[inline]
    fn destination(&mut self) -> &mut Self::W {
        &mut self.inner
    }
}

/// Alias for Interrupt free Printer
pub type InterruptFree<T> = Semihosting<crate::modes::InterruptFree, T>;
/// Alias for Printer without control over interrupts
pub type InterruptOk<T> = Semihosting<crate::modes::InterruptOk, T>;
