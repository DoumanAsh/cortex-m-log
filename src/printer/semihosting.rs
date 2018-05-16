//! Semihosting module

extern crate cortex_m;
extern crate cortex_m_semihosting as sh;

use self::sh::hio;

use ::destination::semihosting::SemihostingComp;
use ::modes::InterruptModer;

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
pub type InterruptFree<T> = Semihosting<::modes::InterruptFree, T>;
/// Alias for Printer without control over interrupts
pub type InterruptOk<T> = Semihosting<::modes::InterruptOk, T>;
