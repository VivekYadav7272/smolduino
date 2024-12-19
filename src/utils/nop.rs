use core::arch::asm;

pub fn nop() {
    // SAFETY: Surely just nop'ing can't have any memory-safety concerns??
    unsafe { asm!("nop") };
}

pub fn nops_n(n: u32) {
    for _ in 0..n {
        nop();
    }
}
