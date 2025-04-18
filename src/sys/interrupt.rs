use core::{
    any::{Any, TypeId},
    arch::asm,
    marker::PhantomData,
};

use crate::sync::synccell::SyncCell;

use super::{mappings::masks, reg_io::Mask};
/*
TODO:
- Implement Interrupts .. DONE
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
pub struct Interrupt<T: Fn() + Sync + 'static> {
    trigger: TriggerType,
    callback: &'static T,
}

// Unfortunately, since drop() is never guaranteed to be called,
// we can't use it to guarantee unsafe invariants
// (like promise to remove reference to a non-'static closure)
// Hence, we're forced to stick with 'static closures only.
impl<T: Fn() + Sync + 'static> Interrupt<T> {
    pub fn new(trigger: TriggerType, callback: &'static T) -> Self {
        Self { trigger, callback }
    }

    pub fn disable(&mut self) {
        let callsite = self.trigger.get_callsite();
        callsite.set(&|| {});
        self.trigger.set_enable_bit(false);
    }

    pub fn enable(&mut self) {
        // first, set the callback.
        let callsite = self.trigger.get_callsite();

        callsite.set(self.callback);
        // depending on different TriggerTypes, enable that specific Trigger's interrupt.
        self.trigger.set_enable_bit(true);
    }
}

impl<T: Fn() + Sync + 'static> Drop for Interrupt<T> {
    fn drop(&mut self) {
        self.disable();
    }
}

/// Create a scope for enabling interrupts and registering callbacks,
/// which are disabled after the scope ends.
/// If there's a nested scope inside a scope, and they try to register
/// a callback on the same interrupt, the most nested interrupt is prioritised.
/// Due to limitations around heap allocations on this system, we cannot maintain
/// a list of applied callbacks, so after the most nested scope ends,
/// the older callback is NOT restored back. The interrupt is simply disabled.
pub fn scope<'scope, T>(scoped: T)
where
    T: FnOnce(&Scope<'scope>),
{
    let scope = Scope {
        scope_lifetime: PhantomData,
    };
    scoped(&scope);
}

// Things to think about: How to store interrupt handlers
// when all closures have different sizes/types?
pub struct Scope<'scope> {
    scope_lifetime: PhantomData<&'scope mut &'scope ()>,
}

impl<'scope> Scope<'scope> {
    // We don't take &'scope self here because
    // that &'a T is a covariant lifetime.
    // &'scope F works because F's lifetime is ATLEAST 'scope, so larger -> smaller in covariancy works.
    // But smaller -> larger cannot ('scope is _slightly_ larger than the
    // lifetime of `|scope| {..}` closure itself)
    // So self.attach() would need to last even longer than the scope it is being called in,
    // which is probably wrong (and throws compiler error).
    pub fn attach<F>(&self, mut trigger: TriggerType, intr: &'scope F)
    // To be debated: &'scope F may not be needed. 'scope constraint here prevents
    // user from using a closure declared inside the `|scope| {..}` closure.
    // But it may be fine to use those closures because the _real_ lifetime of a closure is the
    // minimum lifetime amongst all the variables it captures, and we've already put a constraint for that below.
    //  Closure itself is just code, which is static inside the binary/.text segment.
    // However, as of now, Rust doesn't treat closures with that fine-grained discrimination
    where
        F: Fn() + Sync + 'scope,
    {
        let callsite = trigger.get_callsite();
        // the second pointer cast seems redundant, but it does an implicit cast from &'a to &'static.
        let fn_ptr = intr as &dyn Fn() as *const dyn Fn() as *const dyn Fn();
        // SAFETY: Upon destruction of Scope, this will be cleaned out.
        let fn_static: &'static dyn Fn() = unsafe { &*fn_ptr };
        callsite.set(fn_static);

        let scope_objid = trigger.get_scopeobjid();
        // store our address as a scope identifier of who owns this interrupt.
        scope_objid.set(self.get_id());
        trigger.set_enable_bit(true);
    }

    fn get_id(&self) -> usize {
        self as *const Scope<'scope> as usize
    }
}

/// > But isn't Drop never guaranteed to be called,
/// > so why are you using it to uphold invariancies?
/// >> Well yes in general that is true, however if you
/// >> never mutable access to the Scope object,
/// >> they can't cause shenanigans with it
/// >> (unless of-course they knowingly cause UB and convert
/// >>  a & to a &mut).
impl<'scope> Drop for Scope<'scope> {
    fn drop(&mut self) {
        // loop through all the scope_objids,
        // check which ones are yours and disable their
        // corresponding intr and callback.
        let all_triggers = TriggerType::list_of_all_triggers();

        for mut trigger in all_triggers {
            let scopeid = trigger.get_scopeobjid();
            if scopeid.get() == self.get_id() {
                trigger.set_enable_bit(false);
                let callsite = trigger.get_callsite();
                callsite.set(&|| {});
            }
        }
    }
}

pub enum TriggerType {
    Whatever,
}
impl TriggerType {
    fn set_enable_bit(&mut self, do_enable: bool) {
        todo!("match on the specific kinda interrupt and set its interrupt bit true")
    }

    fn get_callsite(&self) -> &SyncCell<&'static dyn Fn()> {
        &interrupt_vectors::interrupt_fn_1
        // todo!("match on the specific kinda interrupt and return its specific interrupt_fn_$num")
    }

    fn get_scopeobjid(&self) -> &SyncCell<usize> {
        &interrupt_vectors::scope_objids[0]
    }

    fn list_of_all_triggers() -> [Self; 1] {
        [Self::Whatever]
    }
}

mod interrupt_vectors {
    use crate::sync::synccell::SyncCell;
    use crate::utils::nop::nop;

    macro_rules! interrupt_vector {
        ($num:literal $(, $rest:tt)*) => {
            paste::paste! {
                pub static [<interrupt_fn_$num>]: SyncCell<&dyn Fn()> = SyncCell::new(&|| {
                    nop();
                });

                #[no_mangle]
                extern "avr-interrupt" fn [<__vector_ $num>]() {
                    let callback: &dyn Fn() = [<interrupt_fn_$num>].get();
                    callback();
                }
            }

            interrupt_vector!($($rest),*);
        };

        ($name:ident $(, $rest:tt)*) => {
            paste::paste! {
                pub static [<interrupt_fn_$name>]: SyncCell<&dyn Fn()> = SyncCell::new(&|| {
                    nop();
                });

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

    pub static scope_objids: [SyncCell<usize>; 27] = [const { SyncCell::new(0) }; 27];
}
