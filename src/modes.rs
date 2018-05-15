//! Crate configuration modes
extern crate cortex_m;

use self::cortex_m::interrupt;

///Trait to configure interrupt mode.
pub trait InterruptModer {
    #[doc(hidden)]
    fn critical_section<R, F: FnOnce(&interrupt::CriticalSection) -> R>(f: F) -> R;
}

///Ensures that interrupt free execution.
pub struct InterruptFree;
impl InterruptModer for InterruptFree {
    #[inline]
    #[doc(hidden)]
    fn critical_section<R, F: FnOnce(&interrupt::CriticalSection) -> R>(f: F) -> R {
        interrupt::free(f)
    }
}

///No control of interrupts is made.
pub struct InterruptOk;
impl InterruptModer for InterruptOk {
    #[inline]
    #[doc(hidden)]
    fn critical_section<R, F: FnOnce(&interrupt::CriticalSection) -> R>(f: F) -> R {
        f(&unsafe { interrupt::CriticalSection::new() })
    }
}
