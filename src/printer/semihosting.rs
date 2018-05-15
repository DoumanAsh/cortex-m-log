//! Semihosting module

extern crate cortex_m;

use ::destination;
use ::modes::InterruptModer;

use core::marker::PhantomData;

/// Dummy printer
pub struct Semihosting<T: destination::semihosting::SemihostingComp, M: InterruptModer> {
    inner: T,
    _mod: PhantomData<M>
}

impl<T: destination::semihosting::SemihostingComp, M: InterruptModer> Semihosting<T, M> {
    pub fn new(inner: T) -> Self {
        Self { inner, _mod: PhantomData }
    }
}

impl<T: destination::semihosting::SemihostingComp, Mode: InterruptModer> super::Printer for Semihosting<T, Mode> {
    type W = T;
    type M = Mode;

    #[inline]
    fn destination(&mut self) -> &mut Self::W {
        &mut self.inner
    }
}
