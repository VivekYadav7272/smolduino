mod mask_mappings;
mod reg_mappings;

pub mod mappings {
    pub mod regs {
        pub use crate::sys::reg_mappings::*;
    }
    pub mod masks {
        pub use crate::sys::mask_mappings::*;
    }
}
pub mod reg_io;
// This is the default clock freq for Arduino Uno. Someone _CAN_ change it if they want to,
// by messing with the fuse bits. Not handling that right now, maybe later.
pub const F_CPU: u32 = 16_000_000;
