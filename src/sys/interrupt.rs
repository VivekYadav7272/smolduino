use super::{mappings::masks, reg_io::Mask};
use core::arch::asm;
/*
TODO:
- Implement Interrupts
- Implement Cell/RefCell analogues for AVR (basically clearing interrupts for "atomic" operations).. DONE
- Write a setTimeout/setInterval function that utilises this API.
*/

pub struct CriticalSectionGuard {
    was_intr_enabled: bool,
}

impl CriticalSectionGuard {
    pub fn new() -> Self {
        let was_intr_enabled = is_intr_enabled();
        // Case 1: Interrupt was enabled.
        //    Corner case: Soon as we check this, another interrupt comes in and
        //                 preempts us, and disables interrupt globally,
        //                 but then, it must've upon exiting, enabled it back.
        //                 So, the value read here is not stale still.
        // Case 2: Interrupt was disabled.
        //         In this case, nothing can preempt us, so there's no possibility of this being stale.
        if was_intr_enabled {
            disable_intr();
        }
        Self { was_intr_enabled }
    }
}

impl Drop for CriticalSectionGuard {
    fn drop(&mut self) {
        // It is possible that this critical-section was inside another critical-section.
        // In that case, we don't want to re-enable interrupts, as it would allow for nested interrupts
        // and make the parent critical-section not critical anymore,
        // and that violates the safety requirements (set up by ourselves, not an actual AVR requirement).
        if self.was_intr_enabled {
            // SAFETY: Since the interrupt was enabled before, this means that we're not inside another
            // critical section, so it's safe to enable it.
            unsafe {
                enable_intr();
            }
        }
    }
}

pub fn scoped_critical_section<T>(f: impl FnOnce() -> T) -> T {
    let _guard = CriticalSectionGuard::new();
    let x = f();
    // Manually drop, since due to NLL idk if it might drop _guard before x.
    drop(_guard);
    x
}

/// SAFETY Requirements:
/// 1) You must ensure that you're not re-enabling interrupts in the middle of another interrupt
///    as the synchronization primitives are not built to handle nested interrupts.
/// 2) You must ensure that you're not enabling interrupts in the middle of a critical section.
pub unsafe fn enable_intr() {
    unsafe {
        asm!("sei");
    }
}

// I'm not really sure if this one is unsafe, as disabling
// interrupts at best will just have you miss interrupts (no memory unsafeness).
// If however not enabled again manually, may lead to a host of other problems.
pub fn disable_intr() {
    unsafe {
        asm!("cli");
    }
}

pub fn is_intr_enabled() -> bool {
    Mask::with_mask_val(masks::I, 0).read_reg_masked() != 0
}

// Nice, atmega328 already clears the 'I' bit when serving an interrupt,
// so we don't need to worry about nested interrupts.
pub struct Interrupt<T: Fn() + Sync> {
    trigger: TriggerType,
    callback: T,
}

impl<T: Fn() + Sync> Interrupt<T> {
    pub fn new(trigger: TriggerType, callback: T) -> Self {
        Self { trigger, callback }
    }

    pub fn enable(&mut self) {
        self.trigger.set_callsite(&self.callback);
        // depending on different TriggerTypes, enable that specific Trigger's interrupt.
        self.trigger.set_bit(true);
    }
}

impl<T: Fn() + Sync> Drop for Interrupt<T> {
    fn drop(&mut self) {
        // clear the interrupt storage by replacing with a empty closure.
        // the actual closure will be dropped after this function returns.
        self.trigger.set_callsite(&|| {});
        self.trigger.set_bit(false);
    }
}

pub enum TriggerType {}
impl TriggerType {
    fn set_bit(&mut self, enable: bool) {
        todo!("match on the specific kinda interrupt and set its interrupt bit true")
    }

    fn set_callsite(&mut self, fn_ref: &dyn Fn()) {
        let fn_ptr = fn_ref as *const dyn Fn() as *const dyn Fn();
        interrupt_vectors::interrupt_fn_default.set(fn_ptr);
        // todo!("match on the specific kinda interrupt and return its specific interrupt_fn_$num")
    }
}

mod interrupt_vectors {
    #[allow(non_upper_case_globals)]
    use crate::sync::synccell::SyncCell;
    macro_rules! interrupt_vector {
        ($num:literal $(, $rest:tt)*) => {
            paste::paste! {
                pub static [<interrupt_fn_ $num>]: SyncCell<*const dyn Fn()> = SyncCell::new(&|| {});

                #[no_mangle]
                extern "avr-interrupt" fn [<__vector_ $num>]() {
                    let fn_ptr = [<interrupt_fn_ $num>].get();
                    // SAFETY:
                    // The closure will not be modified because it implements Fn(), not FnMut(),
                    // so taking a read-only reference is fine from POV of mutability.
                    // From lifetime POV, we assume that the thing hasn't been destroyed yet.
                    // This invariant is upholded by Interrupt struct, which ensures that when it
                    // is dropped (and the closure with it), then it resets interrupt_fn_test to point to a empty closure.
                    // Hence, we're never pointing to an invalid closure.
                    let fn_ref = unsafe { &*fn_ptr };
                    fn_ref();
                }
            }

            interrupt_vector!($($rest),*);
        };

        ($name:ident $(, $rest:tt)*) => {
            paste::paste! {
                pub static [<interrupt_fn_ $name>]: SyncCell<*const dyn Fn()> = SyncCell::new(&|| {});

                #[no_mangle]
                extern "avr-interrupt" fn [<__vector_ $name>]() {
                    let fn_ptr = [<interrupt_fn_ $name>].get();
                    // SAFETY:
                    // The closure will not be modified because it implements Fn(), not FnMut(),
                    // so taking a read-only reference is fine from POV of mutability.
                    // From lifetime POV, we assume that the thing hasn't been destroyed yet.
                    // This invariant is upholded by Interrupt struct, which ensures that when it
                    // is dropped (and the closure with it), then it resets interrupt_fn_test to point to a empty closure.
                    // Hence, we're never pointing to an invalid closure.
                    let fn_ref = unsafe { &*fn_ptr };
                    fn_ref();
                }
            }
            interrupt_vector!($($rest),*);
        };

        () => {};
    }

    interrupt_vector!(
        default, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26
    );
}
