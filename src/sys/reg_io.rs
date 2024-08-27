use crate::utils::num_traits::UnsignedNumber;
use core::marker::PhantomData;

pub trait RegisterMapping {
    type RegisterType: UnsignedNumber;
    fn get_reg_addr() -> *mut Self::RegisterType;
}

pub trait MaskMapping {
    type Register: RegisterMapping;

    // Technically, mask's type should be RegisterType, but all masks are actually just word-sized (u8)
    // at largest.
    fn get_mask() -> u8;

    fn get_shift() -> u8 {
        let mask = Self::get_mask();
        let shift = mask ^ (mask - 1);
        shift.count_ones() as u8
    }
}

/**
 * SAFETY: Must check if register being passed is actually a register, and if mask is correct or not.
 */
unsafe fn write_reg_unchecked<T: UnsignedNumber>(reg: *mut T, val: T, mask: u8) {
    let reg_val = reg.read_volatile();
    let mask: T = mask.into();
    reg.write_volatile((reg_val & !mask) | (val & mask));
}

pub struct Register<Reg: RegisterMapping> {
    reg: *mut Reg::RegisterType,
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
    // ['ADC', 'ADCH', 'ADCL', 'DDRB', 'DDRC', 'DDRD', 'EEAR', 'EEARH', 'EEARL', 'EEDR', 'GPIOR0',
    // 'GPIOR1', 'GPIOR2', 'ICR1', 'ICR1H', 'ICR1L', 'OCR0A', 'OCR0B', 'OCR1A', 'OCR1AH', 'OCR1AL',
    // 'OCR1B', 'OCR1BH', 'OCR1BL', 'OCR2A', 'OCR2B', 'OSCCAL', 'PCMSK0', 'PCMSK1', 'PCMSK2', 'PINB',
    // 'PINC', 'PIND', 'PORTB', 'PORTC', 'PORTD', 'SP', 'SPDR', 'SPH', 'SPL', 'TCNT0', 'TCNT1',
    // 'TCNT1H', 'TCNT1L', 'TCNT2', 'TWBR', 'TWDR', 'UBRR0', 'UBRR0H', 'UBRR0L', 'UDR0']
    // which don't have available masks, so we don't really know what's "correct" to write there.
    // Most of these require a higher-level abstraction to be correctly/constraintly used anyways,
    // so it's good that having to use unsafe on them feels uneasy and reminds us to create
    // a higher-level abstraction first.
    pub unsafe fn write_reg(&mut self, val: Reg::RegisterType) {
        // SAFETY: User must ensure that the bits being written to are correct or not.
        write_reg_unchecked(self.reg, val, 0xFF);
    }

    pub fn write_reg_mask<Mask: MaskMapping<Register = Reg>>(&mut self, val: Reg::RegisterType) {
        // SAFETY: Using type safety, we've ensured that only allowed bits of a register are written to.
        // Semantically however, it's not assured if the bits written are meaningful or not.
        unsafe { write_reg_unchecked(self.reg, val, Mask::get_mask()) }
    }

    pub fn read_reg(&self) -> Reg::RegisterType {
        // SAFETY: Using type-safety, we know the pointer we're dereferencing is non-null and aligned
        // (naturally aligned because they're all u8)
        unsafe { *self.reg }
    }

    pub fn read_reg_masked<Mask: MaskMapping<Register = Reg>>(&self) -> u8 {
        // SAFETY:
        // 1) Using type-safety (only registers declared in sys::regs), we know the pointer we're dereferencing is non-null and aligned
        // (naturally aligned because they're all u8)
        // 2) We can assert that conversion into u8 will never fail because mask is u8,
        // meaning whatever value we get after and'ing with a mask is a u8.
        unsafe {
            (*self.reg & Mask::get_mask().into())
                .try_into()
                .unwrap_unchecked()
        }
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
        self.reg.write_reg_mask::<M>(val.into());
    }

    pub fn read_val(&self) -> u8 {
        // SAFETY: Through type-safety, we've ensured that the register is correct, the value is placed
        // correctly, and that the mask is correct (obviously)
        self.reg.read_reg_masked::<M>()
    }
}
