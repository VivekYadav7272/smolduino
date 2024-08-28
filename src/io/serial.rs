use crate::sys::{
    self,
    mappings::{masks, regs},
    reg_io::{Mask, MaskMapping, Register},
};

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
        Self::configure_ubrr(baud_rate);
        Self::configure_ucsr();
    }

    /**
     * This register is used to configure the baud rate.
     */
    fn configure_ubrr(baud_rate: u32) {
        // Formula: BaudRate = Freq/(16(UBRR+1)) (with U2X = 0)
        // Formula: BaudRate = Freq/(8(UBRR+1)) (with U2X = 1)
        // i.e, for the same UBRR value, you get twice the baud rate with U2X = 1.

        // We try U2X = 1 mode first, as it has much better error rates for each baud rate
        // compared to U2X = 0 mode.
        let mut ubrr_val: u32 = get_ubrr_val(baud_rate, true);
        // UBRR register only has 12 bits. So if ubrr_val >= 2^12, then we try U2X = 0 instead.

        if !(0u32..1 << 12).contains(&ubrr_val) {
            // SAFETY: U2X0 is a bitfield in UCSRA0 register, hence 1 is a valid value.
            unsafe {
                Mask::<regs::UCSR0A>::new()
                    .add_masked_val(masks::U2X0, 1)
                    .write_val()
            };
            ubrr_val = get_ubrr_val(baud_rate, false);
        }

        // Also remember, we're using UBRRL and UBRRH only.
        // UBRRH and UCSRC register share the same address.
        // So, you need to clear URSEL bit to indicate you're using UBRRH and NOT UCSRC.
        // |URSEL| -- | -- | -- | UBRR[11:8]| <-- UBRR0H
        // |            UBRR[7:0]           | <-- UBRR0L
        // We shouldn't need to do this as the value calculated should NOT exceed the 12 bits provided
        // for UBRR, meaning URSEL should already be zero. But doing it explicitly is better than not doing it.
        let ubrr_val = ubrr_val & !0x8000;

        let mut ubrr = Register::<regs::UBRR0>::new();
        // SAFETY: UBRR has been calculated using the formula in datasheet,
        // hence it has to be a legal value.
        unsafe { ubrr.write_reg(ubrr_val as u16) };
    }

    /**
     * This register is used to configure IRQ, contains the status bit of if data has arrived, etc.
     */
    fn configure_ucsr() {
        // For UCSRA register
        // --- NOTHING TO DO FOR NOW, will be used when sending/receiving packets --
        // U2X0 already handled in configure_ubrr() according to baud rate rquired

        // For UCSRB register

        // SAFETY: Masks can only be added if they belong to the register, and they can only have their legal
        // value.
        let mut ucsrb_val = Mask::<regs::UCSR0B>::new();
        unsafe {
            ucsrb_val
                .add_masked_val(masks::RXCIE0, 1)
                .add_masked_val(masks::UDRIE0, 1)
                .add_masked_val(masks::RXEN0, 1)
                .add_masked_val(masks::TXEN0, 1)
                .write_val();
        }

        // For UCSRC register

        let mut ucsrc_val = Mask::<regs::UCSR0C>::new();
        ucsrc_val
            // set URSEL bit (7) to 1 (because URSEL=0 implies we're dealing with UBRR0H)
            // leave UMSEL bit to 0
            // (URSEL0's mask is also UMSEL0, := 0xC0)
            .add_masked_val(masks::UMSEL0, 0b10)
            // leave both UPM bits as parity as of now is not required
            // leave USBS (bit 3) to 0 since we want 1 stop bit
            // set UCSZ to 0b11 to show we want a packet to be 8-bits long.
            .add_masked_val(masks::UCSZ0, 0b11);

        // SAFETY: Mask has legal values bruv
        unsafe { Register::<regs::UCSR0C>::new().write_reg_masked(&ucsrc_val) };
    }
}

fn get_ubrr_val(baud_rate: u32, u2x_enabled: bool) -> u32 {
    // => UBRR = Freq/(16*BaudRate) - 1 (with U2X = 0)
    // => UBRR = Freq/(8*BaudRate) - 1 (with U2X = 1)
    if u2x_enabled {
        sys::F_CPU / (8 * baud_rate) - 1
    } else {
        sys::F_CPU / (16 * baud_rate) - 1
    }
}
