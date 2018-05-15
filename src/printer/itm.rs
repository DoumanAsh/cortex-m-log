//! Itm module

extern crate cortex_m;

use ::destination;
use ::modes::InterruptModer;

use core::marker::PhantomData;

/// Dummy printer
pub struct Itm<M: InterruptModer> {
    inner: destination::Itm,
    _mod: PhantomData<M>
}

impl<M: InterruptModer> Itm<M> {
    pub fn new(itm: destination::Itm) -> Self {
        Self { inner: itm, _mod: PhantomData }
    }
}

impl<Mode: InterruptModer> super::Printer for Itm<Mode> {
    type W = destination::Itm;
    type M = Mode;

    #[inline]
    fn destination(&mut self) -> &mut Self::W {
        &mut self.inner
    }
}
