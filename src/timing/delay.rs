use crate::sys::{
    self,
    mappings::{masks, regs},
    reg_io::{Mask, Register},
};

enum TimerPrecision {
    Imprecise,
    Precise,
    VeryPrecise, // 1:1 with system clk
}

impl TimerPrecision {
    pub fn timer_mask(&self) -> Mask<regs::TCCR1B> {
        Mask::with_mask_val(
            masks::CS1,
            match self {
                TimerPrecision::Imprecise => 0b101,
                TimerPrecision::Precise => 0b100,
                TimerPrecision::VeryPrecise => 0b1,
            },
        )
    }

    pub fn prescale_factor(&self) -> u32 {
        match self {
            TimerPrecision::Imprecise => 1024,
            TimerPrecision::Precise => 256,
            TimerPrecision::VeryPrecise => 1,
        }
    }
}
/**
 * No time calculation. Just sleep for `ticks` amount of ticks.
 */
pub fn delay_ticks(ticks: u128) {
    // Scale clock down by a factor of 1024. (i.e 1024 system ticks == 1 timer tick)
    _delay_ticks(ticks, TimerPrecision::Imprecise)
}

pub fn delay(duration: core::time::Duration) {
    let delay_millis = duration.as_millis();

    // I was initially going to have different pre-scaling options
    // based on the delay given. But it sounds like there's enough code involved,
    // that anything less than 1ms, is likely not gonna be accurately delayed.
    // MAYBE we do that in future.
    let precision = TimerPrecision::Imprecise; //get_prescaler(delay_millis);
    let scaled_clk = sys::F_CPU / precision.prescale_factor();
    // find out how many clks to "sleep" for.
    let num_ticks = (scaled_clk as u128 * delay_millis) / 1000;
    _delay_ticks(num_ticks, precision)
}

fn _delay_ticks(ticks: u128, precision: TimerPrecision) {
    // SAFETY: You must set the prescaler before starting the counter.
    unsafe { precision.timer_mask().write_val() };

    let mut counter = Register::<regs::TCNT1>::new();
    // initialised with 1 because we want to write 1 at that bit later on
    let mut counter_overflow_bit = Mask::with_mask_val(masks::TOV1, 1);
    let counter_max_val = (1u128 << counter.width()) - 1;

    for _ in 0..ticks / counter_max_val {
        // let it count to the full width of the register.
        // SAFETY: Counter register. All values are legal.
        unsafe { counter.write_reg(0) };

        while counter_overflow_bit.read_reg_masked() == 0 {}

        // SAFETY: Overflow is reset to zero, by writing a logical one to it (weird ikr).
        unsafe { counter_overflow_bit.write_val() };
    }

    let remaining_ticks = ticks % counter_max_val;

    // SAFETY: Counter register. All values are legal.
    unsafe { counter.write_reg(u16::MAX - remaining_ticks as u16) };

    while counter_overflow_bit.read_reg_masked() == 0 {}

    // SAFETY: Overflow is reset to zero, by writing a logical one to it (weird ikr).
    unsafe { counter_overflow_bit.write_val() };
}
