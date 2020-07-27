//! Itm module

use cortex_m::interrupt::Mutex;

use crate::destination;
use crate::modes::InterruptModer;

use core::marker::PhantomData;
use core::fmt::{self, Write};

/// ITM backed printer
pub struct Itm<M: InterruptModer> {
    inner: destination::Itm,
    _mod: PhantomData<M>
}

impl<M: InterruptModer> Itm<M> {
    ///Constructs new instance
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

/// ITM backed printer with `Sync`
pub struct ItmSync<M: InterruptModer> {
    inner: destination::Itm,
    lock: Mutex<()>,
    _mod: PhantomData<M>,
}

impl<M: InterruptModer> ItmSync<M> {
    ///Constructs new instance
    pub fn new(itm: destination::Itm) -> Self {
        Self { inner: itm, lock: Mutex::new(()), _mod: PhantomData }
    }
}

impl<Mode: InterruptModer> super::Printer for ItmSync<Mode> {
    type W = destination::Itm;
    type M = Mode;

    #[inline]
    fn destination(&mut self) -> &mut Self::W {
        &mut self.inner
    }

    #[inline]
    ///Prints formatted output to destination
    fn print(&mut self, args: fmt::Arguments) {
        cortex_m::interrupt::free(|cs| {
            let _lock = self.lock.borrow(cs);
            let _ = self.inner.write_fmt(args);
        });
    }

    #[inline]
    ///Prints formatted output to destination with newline
    fn println(&mut self, args: fmt::Arguments) {
        cortex_m::interrupt::free(|cs| {
            let _lock = self.lock.borrow(cs);
            let _ = self.inner.write_fmt(args);
            let _ = self.inner.write_str("\n");
        });
    }
}

unsafe impl<Mode: InterruptModer> Sync for ItmSync<Mode> {}

/// ITM backed printer with the assumption of `Sync`
/// This does *not* actually `Sync`'ronize access to the ITM
/// For proper `Sync`, use `ItmSync`.
pub struct ItmAssumeSync {
    inner: destination::Itm,
}

impl ItmAssumeSync {
    ///Constructs new instance
    pub unsafe fn new(itm: destination::Itm) -> Self {
        Self { inner: itm }
    }
}

impl super::Printer for ItmAssumeSync {
    type W = destination::Itm;
    type M = crate::modes::InterruptOk;

    #[inline]
    fn destination(&mut self) -> &mut Self::W {
        &mut self.inner
    }

    #[inline]
    ///Prints formatted output to destination
    fn print(&mut self, args: fmt::Arguments) {
        let _ = self.inner.write_fmt(args);
    }

    #[inline]
    ///Prints formatted output to destination with newline
    fn println(&mut self, args: fmt::Arguments) {
            let _ = self.inner.write_fmt(args);
            let _ = self.inner.write_str("\n");
    }
}

unsafe impl Sync for ItmAssumeSync {}

/// Alias for Interrupt free Printer
pub type InterruptFree = Itm<crate::modes::InterruptFree>;
/// Alias for Printer without control over interrupts
pub type InterruptOk = Itm<crate::modes::InterruptOk>;
/// Alias for Synced Itm.
pub type InterruptSync = ItmSync<crate::modes::InterruptFree>;
