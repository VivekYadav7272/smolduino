use crate::error::Error;
use crate::sys::interrupt::CriticalSectionGuard;
use core::marker::PhantomData;
use core::ptr;
use core::result::Result;
use core::{cell::UnsafeCell, ops::Deref};

// TODO: Also implement "Sync" RefCell, which I now realise is just a Mutex.
pub struct Mutex<T> {
    // INVARIANT: DO NOT touch this field UNTIL YOU ARE IN A CRITICAL SECTION
    already_acquired: UnsafeCell<bool>,
    data: UnsafeCell<T>,
}
// unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Mutex {
            already_acquired: UnsafeCell::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<T>, Error> {
        let _guard = CriticalSectionGuard::new();
        // SAFETY: In critical section, so we currently are the only readers.
        let already_acquired = unsafe { ptr::read(self.already_acquired.get()) };
        if already_acquired {
            // I assume _guard is gonna be dropped at this line right?
            return Err(Error::LockAlreadyAcquiredError);
        }
        // else, we're gonna get the lock on it.
        // SAFETY: In critical section, so we currently are the only writer.
        unsafe {
            ptr::write(self.already_acquired.get(), true);
        }

        Ok(MutexGuard {
            guarded_mutex: self,
            cs_guard: _guard,
        })
    }
}

pub struct MutexGuard<'a, T> {
    guarded_mutex: &'a Mutex<T>,
    // Two choices here: Either let the interrupts
    // come in, check that already_acquired is true, and then do, whatever the f they want.
    // That is _kinda_ away from the spirit of Mutexes as we experience them on hosted systems,
    // where if u can't get the lock, u just wait.
    // So I'd like for interrupts to just not be a thing, while the lock is held.
    // and then they are "awakened", after the lock is released.
    cs_guard: CriticalSectionGuard,
}
