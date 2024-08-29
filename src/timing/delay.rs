use crate::utils::nop::nop;

pub fn delay(x: u32) {
    for _ in 0..x {
        nop();
    }
}
