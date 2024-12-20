use core::arch::asm;

use super::{mappings::masks, reg_io::Mask};

/*
TODO:
- Implement Interrupts
- Implement Cell/RefCell analogues for AVR (basically clearing interrupts for "atomic" operations)
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
        // depending on different TriggerTypes, enable that specific Trigger's interrupt.
        let intr_bit_mask = self.trigger.intr_bit_mask();
    }
}

pub enum TriggerType {}
impl TriggerType {
    pub fn intr_bit_mask(&self) -> u8 {
        todo!("match on the specific kinda interrupt and return the bit mask accordingly")
    }
}

mod interrupt_vectors {
    use crate::utils::nop::nops_n;

    macro_rules! interrupt_vector {
        ($num:literal $(, $rest:tt)*) => {
            paste::paste! {
                #[no_mangle]
                extern "avr-interrupt" fn [<__vector_ $num>]() {
                    // nops_n($num);
                }
            }

            interrupt_vector!($($rest),*);
        };

        ($name:ident $(, $rest:tt)*) => {
            paste::paste! {
                #[no_mangle]
                extern "avr-interrupt" fn [<__vector_ $name>]() {
                    // nops_n(69);
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
