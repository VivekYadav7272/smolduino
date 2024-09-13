use crate::sys::{
    self,
    mappings::{masks, regs},
    reg_io::{Mask, Register},
};

/**
 * No time calculation. Just sleep for `ticks` amount of ticks.
 */
pub fn delay_ticks(ticks: u128) {
    // Scale clock down by a factor of 1024. (i.e 1024 system ticks == 1 timer tick)
    let timing_control = Mask::with_mask_val(masks::CS1, 0b101);

    _delay_ticks(ticks, timing_control)
}

pub fn delay(duration: core::time::Duration) {
    let delay_millis = duration.as_millis();

    let (timing_control, scaled_clk) = get_prescaler(delay_millis);

    // find out how many clks to "sleep" for.
    let num_ticks = (scaled_clk as u128 * delay_millis) / 1000;
    _delay_ticks(num_ticks, timing_control)
}

fn get_prescaler(_delay: u128) -> (Mask<regs::TCCR1B>, u32) {
    // I was initially going to have different pre-scaling options
    // based on the delay given. But it sounds like there's enough code involved,
    // that anything less than 1ms, is likely not gonna be accurately delayed.
    // MAYBE we do that in future.
    (Mask::with_mask_val(masks::CS1, 0b101), sys::F_CPU / 1024)
}

fn _delay_ticks(ticks: u128, mut timing_control: Mask<regs::TCCR1B>) {
    // SAFETY: You must set the prescaler before starting the counter.
    unsafe { timing_control.write_val() };

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
    unsafe { counter.write_reg(remaining_ticks as u16) };

    while counter_overflow_bit.read_reg_masked() == 0 {}

    // SAFETY: Overflow is reset to zero, by writing a logical one to it (weird ikr).
    unsafe { counter_overflow_bit.write_val() };
}
