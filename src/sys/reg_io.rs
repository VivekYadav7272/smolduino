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
        (shift.count_ones() - 1) as u8
    }
}

/**
 * SAFETY: Must check if register being passed is actually a register, and if mask is correct or not.
 */
unsafe fn write_reg_unchecked<T: UnsignedNumber>(reg: *mut T, val: T, mask: T) {
    let reg_val = reg.read_volatile();
    reg.write_volatile((reg_val & !mask) | (val & mask));
}

pub struct Register<Reg: RegisterMapping> {
    // We could've not stored this field and just called Reg::get_reg_addr()
    // everytime we needed to. However, ironically imo that would cause more code bloat
    // (all functions using Reg::get_reg_addr() now necessarily need to have one instance for each Reg)
    // This way, those functions only need to generate one code that deals with `self.reg`.
    // This still does generate code for each Reg::RegisterType, but in practice there are only two -
    // u8 and u16.
    reg: *mut Reg::RegisterType,
    _marker: PhantomData<Reg>,
}

impl<Reg: RegisterMapping> Register<Reg> {
    // MAYBE: Compiler probably inlines this function, meaning we don't have multiple instances
    // of `Self::new()` for each Reg type.
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
        let bits = core::mem::size_of::<Reg::RegisterType>() as u32 * 8;
        let mask = (1u32 << bits) - 1;

        // SAFETY: Should be fine because the mask should only be exactly as large as RegisterType.
        let mask: Reg::RegisterType = unsafe { mask.try_into().unwrap_unchecked() };
        // SAFETY: User must ensure that the bits being written to are correct or not.
        unsafe { write_reg_unchecked(self.reg, val, mask.into()) };
    }

    pub unsafe fn write_reg_masked(&mut self, mask: &Mask<Reg>) {
        // SAFETY: Must ensure if the bits written are meaningful or not.
        unsafe { write_reg_unchecked(self.reg, mask.get_val().into(), mask.get_mask().into()) }
    }

    pub fn read_reg(&self) -> Reg::RegisterType {
        // SAFETY: All pointers/structs defined in regs::* are non-null and aligned.
        unsafe { self.reg.read_volatile() }
    }

    pub fn read_reg_masked(&self, mask: &Mask<Reg>) -> u8 {
        // SAFETY:
        // 1) Using type-safety (only registers declared in sys::regs), we know the pointer we're dereferencing is non-null and aligned
        // (naturally aligned because they're all u8)
        // 2) We can assert that conversion into u8 will never fail because mask is u8,
        // meaning whatever value we get after and'ing with a mask is a u8.
        unsafe {
            (self.reg.read_volatile() & mask.get_mask().into())
                .try_into()
                .unwrap_unchecked()
        }
    }
}

pub struct Mask<Reg: RegisterMapping> {
    val: u8,
    mask: u8,
    reg: PhantomData<Reg>,
}

impl<Reg: RegisterMapping> Mask<Reg> {
    pub fn new() -> Self {
        Self {
            val: 0,
            mask: 0,
            reg: PhantomData,
        }
    }

    pub fn with_mask_val<M: MaskMapping<Register = Reg>>(mask: M, val: u8) -> Self {
        let mut me = Self::new();
        me.add_mask_val(mask, val);
        me
    }

    pub fn add_mask_val<M: MaskMapping<Register = Reg>>(&mut self, _mask: M, val: u8) -> &mut Self {
        self.val |= (val << M::get_shift()) & M::get_mask();
        self.mask |= M::get_mask();
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.val = 0;
        self.mask = 0;
        self
    }

    /**
     * Writes the value with the mask accumulated till now, into the register.
     */
    pub unsafe fn write_val(&mut self) {
        // SAFETY: Same safety implication as Register::write_reg_masked
        unsafe { Register::<Reg>::new().write_reg_masked(&*self) };
    }

    pub fn add_val(&mut self, val: u8) -> &mut Self {
        self.val |= val;
        self
    }

    pub fn add_mask<M: MaskMapping<Register = Reg>>(&mut self, _mask: M) -> &mut Self {
        self.mask |= M::get_mask();
        self
    }

    pub fn read_reg_masked(&self) -> u8 {
        // SAFETY: Through type-safety, we've ensured that the register is correct, the value is placed
        // correctly, and that the mask is correct (obviously)
        Register::<Reg>::new().read_reg_masked(self)
    }

    pub fn get_val(&self) -> u8 {
        self.val
    }

    pub fn get_mask(&self) -> u8 {
        self.mask
    }

    pub fn get_shift(&self) -> u8 {
        let mask = self.get_mask();
        let shift = mask ^ (mask - 1);
        (shift.count_ones() - 1) as u8
    }
}
