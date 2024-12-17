use crate::sys::interrupt;
use core::{cell::UnsafeCell, ptr};

// Thankfully, AVR disables global interrupts when an interrupt is being served,
// meaning that we don't need to handle nested interrupts.

#[derive(Debug)]
pub struct SyncCell<T: ?Sized>(UnsafeCell<T>);
// SAFETY: Since Atmega328P is a single-"core" microcontroller,
// and we've guarded against interrupts, hence it's effectively "shareable".
unsafe impl<T: ?Sized> Sync for SyncCell<T> {}
impl<T: Copy> SyncCell<T> {
    pub fn get(&self) -> T {
        let was_enabled = interrupt::is_intr_enabled();
        // We don't want to get interrupted in the middle of copying the value
        interrupt::disable_intr();

        // SAFETY: We're not going to be interrupted, so it's safe to read the value.
        // + T: Copy means that we don't need to move the value out of the cell.
        let val = unsafe { ptr::read(self.0.get()) };

        if was_enabled {
            // SAFETY: We're not in another interrupt, and critical section is over, so it's safe to
            // enable it again now.
            unsafe { interrupt::enable_intr() };
        }
        val
    }
}

impl<T: Copy> Clone for SyncCell<T> {
    fn clone(&self) -> Self {
        Self::new(self.get())
    }
}

impl<T: Default> Default for SyncCell<T> {
    /// Creates a `Cell<T>`, with the `Default` value for T.
    #[inline]
    fn default() -> SyncCell<T> {
        SyncCell::new(Default::default())
    }
}

impl<T: PartialEq + Copy> PartialEq for SyncCell<T> {
    fn eq(&self, other: &SyncCell<T>) -> bool {
        self.get() == other.get()
    }
}
impl<T: Eq + Copy> Eq for SyncCell<T> {}

impl<T: PartialOrd + Copy> PartialOrd for SyncCell<T> {
    fn partial_cmp(&self, other: &SyncCell<T>) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(&other.get())
    }
}
impl<T: Ord + Copy> Ord for SyncCell<T> {
    #[inline]
    fn cmp(&self, other: &SyncCell<T>) -> core::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}
impl<T> From<T> for SyncCell<T> {
    /// Creates a new `Cell<T>` containing the given value.
    fn from(t: T) -> SyncCell<T> {
        SyncCell::new(t)
    }
}

impl<T> SyncCell<T> {
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    pub fn swap(&self, other: T) -> T {
        let was_enabled = interrupt::is_intr_enabled();
        // We don't want to get interrupted in the middle of copying the value
        interrupt::disable_intr();

        // SAFETY: Safe to do so since not going to be interrupted
        // + inner value is valid for both read and write
        // + inner value must've been properly aligned by default
        // + no way to create inner value without it being a proper T (hence initialised)
        let old_val = unsafe { ptr::replace(self.0.get(), other) };

        if was_enabled {
            // SAFETY: We're not in another interrupt, and critical section is over, so it's safe to
            // enable it again now.
            unsafe { interrupt::enable_intr() };
        }
        old_val
    }

    pub fn set(&self, value: T) {
        self.swap(value);
    }
}
// TODO: Also implement RefCell.
