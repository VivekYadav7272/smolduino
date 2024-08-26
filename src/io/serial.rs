use crate::sys::{self, reg_io::Register, regs};

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

        let mut ubrrl = Register::<regs::UBRR0L>::new();

        // SAFETY: ubrr_val has been calculated using the formula given.
        unsafe { ubrrl.write_reg((ubrr_val & 0xFF) as u8) };

        let mut ubrrh = Register::<regs::UBRR0H>::new();

        // SAFETY: ubrr_val has been calculated using the formula given.
        unsafe { ubrrh.write_reg(((ubrr_val >> 8) & 0xFF) as u8) };
    }
}
