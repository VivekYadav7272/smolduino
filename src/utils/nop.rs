use core::arch::asm;

pub fn nop() {
    // SAFETY: Surely just nop'ing can't have any memory-safety concerns??
    unsafe { asm!("nop") };
}
