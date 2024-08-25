#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use avrd::current;
use core::arch::asm;
use core::{hint::black_box, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn delay(x: u32) {
    for _ in 0..x {
        unsafe {
            asm!("nop");
        }
    }
}

unsafe fn write_reg(reg: *mut u8, val: u8, mask: u8) {
    let reg_val = reg.read_volatile();
    reg.write_volatile((reg_val & !mask) | (val & mask));
}

#[no_mangle]
extern "C" fn main() -> ! {
    const LED_BUILTIN: u8 = 5;
    unsafe {
        let portB_data_direction = current::DDRB;
        // set it to output mode
        write_reg(portB_data_direction, 1 << LED_BUILTIN, 1 << LED_BUILTIN);
        let portB = current::PORTB;

        loop {
            write_reg(portB, 1 << LED_BUILTIN, 1 << LED_BUILTIN);
            delay(500_000);
            write_reg(portB, 0, 1 << LED_BUILTIN);
            delay(500_000);
        }
    }
}
