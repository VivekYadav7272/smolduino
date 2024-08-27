use crate::sys::{self, mappings::regs, reg_io::Register};

/**
 * Utilises the hardware UART capabilities of Atmega328p to do serial communication.
 */
pub struct Serial {
    baud_rate: u32,
}

impl Default for Serial {
    fn default() -> Self {
        const DEFAULT_BAUD_RATE: u32 = 9600;
        Self::init(DEFAULT_BAUD_RATE);
        Self {
            baud_rate: DEFAULT_BAUD_RATE,
        }
    }
}

impl Serial {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_baud_rate(baud_rate: u32) -> Self {
        Self::init(baud_rate);
        Self { baud_rate }
    }

    fn init(baud_rate: u32) {
        // Write to UBRR
        // Formula: BaudRate = Freq/(16(UBRR+1))
        // => UBRR = Freq/(16*BaudRate) - 1

        let ubrr_val: u32 = sys::F_CPU / (16 * baud_rate) - 1;

        let mut ubrr = Register::<regs::UBRR0>::new();
        // SAFETY: UBRR has been calculated using the formula in datasheet,
        // hence it has to be a legal value.
        unsafe { ubrr.write_reg(ubrr_val as u16) };
    }
}
