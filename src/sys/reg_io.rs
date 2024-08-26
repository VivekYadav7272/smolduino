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

    // Required for registers
    // ['ADCH', 'ADCL', 'DDRB', 'DDRC', 'DDRD', 'EEARH', 'EEARL', 'EEDR', 'GPIOR0', 'GPIOR1',
    // 'GPIOR2', 'ICR1H', 'ICR1L', 'OCR0A', 'OCR0B', 'OCR1AH', 'OCR1AL', 'OCR1BH', 'OCR1BL',
    // 'OCR2A', 'OCR2B', 'OSCCAL', 'PCMSK0', 'PCMSK1', 'PCMSK2', 'PINB', 'PINC', 'PIND', 'PORTB',
    // 'PORTC', 'PORTD', 'SPDR', 'SPH', 'SPL', 'TCNT0', 'TCNT1H', 'TCNT1L', 'TCNT2', 'TWBR', 'TWDR',
    // 'UBRR0H', 'UBRR0L', 'UDR0']
    // which don't have available masks, so we don't really know what's "correct" to write there.
    // Most of these require a higher-level abstraction to be correctly/constraintly used anyways,
    // so it's good that having to use unsafe on them feels uneasy and reminds us to create
    // a higher-level abstraction first.
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

    pub fn read_reg_masked<Mask: MaskMapping<Register = Reg>>(&self) -> u8 {
        unsafe { *self.reg & Mask::get_mask() }
    }
}

pub struct Mask<Reg: RegisterMapping, M: MaskMapping<Register = Reg>> {
    reg: Register<Reg>,
    _marker: PhantomData<M>,
}

impl<Reg, M> Mask<Reg, M>
where
    Reg: RegisterMapping,
    M: MaskMapping<Register = Reg>,
{
    pub fn new() -> Self {
        Self {
            reg: Register::new(),
            _marker: PhantomData,
        }
    }

    pub fn write_val(&mut self, val: u8) {
        let val = val << M::get_shift();
        self.reg.write_reg_mask::<M>(val);
    }

    pub fn read_val(&self) -> u8 {
        // SAFETY: Through type-safety, we've ensured that the register is correct, the value is placed
        // correctly, and that the mask is correct (obviously)
        self.reg.read_reg_masked::<M>()
    }
}
