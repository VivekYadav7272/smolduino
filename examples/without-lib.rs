#![no_std]
#![no_main]

use core::{hint::black_box, panic::PanicInfo};

use avrd::atmega328p;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    const LED_BUILTIN: u8 = 5;
    unsafe {
        let port_b_data_direction = atmega328p::DDRB;
        // set it to output mode
        *port_b_data_direction |= 1 << LED_BUILTIN;
        let port_b = atmega328p::PORTB;

        loop {
            *port_b ^= 1 << LED_BUILTIN;
            for i in 0..500_000 {
                black_box(i);
            }
            *port_b ^= 1 << LED_BUILTIN;
            for i in 0..500_000 {
                black_box(i);
            }
        }
    }
}
