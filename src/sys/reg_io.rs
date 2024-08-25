use core::marker::PhantomData;

use super::reg_mappings::{MaskMapping, RegisterMapping};

/**
 * SAFETY: Must check if register being passed is actually a register, and if mask is correct or not.
 */
unsafe fn write_reg_unchecked(reg: *mut u8, val: u8, mask: u8) {
    let reg_val = reg.read_volatile();
    reg.write_volatile((reg_val & !mask) | (val & mask));
}

pub struct Register<Reg: RegisterMapping> {
    reg: *mut u8,
    _marker: PhantomData<Reg>,
}

impl<Reg: RegisterMapping> Register<Reg> {
    pub fn new() -> Self {
        Self {
            reg: Reg::get_reg_addr(),
            _marker: PhantomData::<Reg>,
        }
    }

    pub unsafe fn write_reg(&mut self, val: u8) {
        // SAFETY: User must ensure that the bits being written to are correct or not.
        write_reg_unchecked(self.reg, val, 0xFF);
    }

    pub fn write_reg_mask<Mask: MaskMapping<Register = Reg>>(&mut self, val: u8) {
        // SAFETY: Using type safety, we've ensured that only allowed bits of a register are written to.
        // Semantically however, it's not assured if the bits written are meaningful or not.
        unsafe { write_reg_unchecked(self.reg, val, Mask::get_mask()) }
    }

    pub fn read_reg(&self) -> u8 {
        // SAFETY: Using type-safety, we know the pointer we're dereferencing is non-null and aligned
        // (naturally aligned because they're all u8)
        unsafe { *self.reg }
    }
}
