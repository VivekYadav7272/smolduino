use crate::{error::Error, sys::interrupt::CriticalSectionGuard};
use core::{cell::UnsafeCell, ops::Deref, ops::DerefMut, ptr, result::Result};

// TODO: Also implement "Sync" RefCell, which I now realise is just a Mutex.
pub struct Mutex<T> {
    // INVARIANT: DO NOT touch this field UNTIL YOU ARE IN A CRITICAL SECTION
    already_acquired: UnsafeCell<bool>,
    data: UnsafeCell<T>,
}
// Mutex data might be mem::replace'd, so it needs to be Send.
// Sidenote, I don't think on this microcontroller `Send` really has any meaning
// because interrupts (the only other "thread") don't really have a need to be "sent" anywhere.
unsafe impl<T: Send> Sync for Mutex<T> {}

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
            _cs_guard: _guard,
        })
    }
}

pub struct MutexGuard<'a, T> {
    // Why? Because if we kept a direct UnsafeCell<T>, the compiler would not have an attached
    // lifetime to it, which means it can't track that MutexGuard should live only as long as Mutex
    // Also, because of the invariant of Mutex, if u get to the point of modifying UnsafeCell<T>,
    // you're already the only one, so it's fine (interior mutability is allowed via a & reference).
    guarded_mutex: &'a Mutex<T>,
    // Two choices here: Either let the interrupts
    // come in, check that already_acquired is true, and then do, whatever the f they want.
    // That is _kinda_ away from the spirit of Mutexes as we experience them on hosted systems,
    // where if u can't get the lock, u just wait.
    // So I'd like for interrupts to just not be a thing, while the lock is held.
    // and then they are "awakened", after the lock is released.
    // This does have an unfortunate side effects
    _cs_guard: CriticalSectionGuard,
}

impl<T: Send> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Holding a MutexGuard means we're the only ones with access, so
        // safe to give a reference to the data.
        unsafe { &*self.guarded_mutex.data.get() }
    }
}

impl<T: Send> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Holding a MutexGuard means we're the only ones with access, so
        // safe to give an exclusive reference to the data.
        unsafe { &mut *self.guarded_mutex.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // SAFETY: In critical section, so we currently are the only writer.
        unsafe {
            ptr::write(self.guarded_mutex.already_acquired.get(), false);
        }

        // CriticalSectionGuard destroyed only after this function (this is &mut self)
    }
}
