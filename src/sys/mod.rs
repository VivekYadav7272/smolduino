pub mod mask_mappings;
pub mod reg_io;
pub mod reg_mappings;

pub use mask_mappings as masks;
pub use reg_mappings as regs;
// This is the default clock freq for Arduino Uno. Someone _CAN_ change it if they want to,
// by messing with the fuse bits. Not handling that right now, maybe later.
pub const F_CPU: u32 = 16000000;
