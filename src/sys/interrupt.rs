use core::arch::asm;

/*
Todo:
- Implement Interrupts
- Implement Cell/RefCell analogues for AVR (basically clearing interrupts for "atomic" operations)
- Write a setTimeout/setInterval function that utilises this API.
 */

fn enable_intr() {
    unsafe {
        asm!("sei");
    }
}

fn disable_intr() {
    unsafe {
        asm!("cli");
    }
}
// Nice, atmega328 already clears the 'I' bit when serving an interrupt,
// so we don't need to worry if another interrupt might interrupt our interrupt.
pub struct Interrupt<T: Fn() -> ()> {
    trigger: TriggerType,
    callback: T,
}

impl<T: Fn() -> ()> Interrupt<T> {
    pub fn new(trigger: TriggerType, callback: T) -> Self {
        Self { trigger, callback }
    }

    pub fn enable(&mut self) {
        // depending on different TriggerTypes, enable that specific Trigger's interrupt.
        let intr_bit_mask = self.trigger.intr_bit_mask();
    }
}

pub enum TriggerType {}
impl TriggerType {
    pub fn intr_bit_mask(&self) -> u8 {
        todo!("match on the specific kinda interrupt and return the bit mask accordingly")
    }
}
