use crate::{error::Error, sys::interrupt::CriticalSectionGuard};
use core::{cell::UnsafeCell, ops::Deref, ops::DerefMut, ptr, result::Result};

pub struct Mutex<T> {
    already_acquired: UnsafeCell<bool>,
    data: UnsafeCell<T>,
}
// Mutex data might be mem::replace'd, so it needs to be Send.
// Sidenote, I don't think on this microcontroller `Send` really has any meaning
// because interrupts (the only other "thread") don't really have a need to be "sent" anywhere.
// I think this might still be required for `core` types that expected `Send` not being
// implemented meaning that they are safe to access concurrently.
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
    // Two choices here:
    // 1) Either let the interrupts be allowed to wake up
    //    even if the lock is held, and then if they try to acquire
    //    the lock, they'll just get an error.
    //    Obvious problem: Wtf are the interrupts supposed to do then? Just spinlock?
    //    that's not viable on a single-core microcontroller (not because of performance,
    //    but because this is effectively a livelock in our case).
    //    But we do give the interrupts more freedom to handle this the way they want --
    //    either skip out on their main code flow path, or temporarily store whatever
    //    they were going to do in some extra buffer, which the "main" thread then checks
    //    later on (pretty finicky imo, bad way to handle).
    // 2) But this adds a lot of complexity to the code, making all interrupts
    //    think about if they might be stepping on a mutex, and then having to handle that.
    //    So alternative method (which is the current implementation), just block the interrupts
    //    while the guard is held. This massively simplifies the cognitive model (all interrupts
    //    can safely consider that mutexes they care about are unlocked).
    //    Also matches the mental model of how Mutexes behave on hosted systems.
    //    DOWNSIDE: If the main thread is doing something that takes a long time, interrupts
    //    will not be active for a long time. And not only that, current implementation blocks
    //    ALL interrupts as of now, so even non-mutex interrupts are blocked.
    //    Can be bad for interrupts that aren't 'sticky'.
    _cs_guard: CriticalSectionGuard,
}

impl<T: Send> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Holding a MutexGuard means we're the only ones with access, so
        // safe to give a reference to the data.
        // Also the data is a normal T, so invariants for creating a reference are satisfied.
        // Also, the lifetime of this reference is tied to the lifetime of the MutexGuard,
        // so it can't be that you stuff this reference somewhere cheekily and then MutexGuard dies, then
        // some other guy comes in, does the same and now you have two potential references
        // to thread-sensitive data.
        unsafe { &*self.guarded_mutex.data.get() }
    }
}

impl<T: Send> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Same invariants as for .deref()
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
