#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

pub mod io;
pub(crate) mod sys;
pub mod utils;

use core::arch::asm;
use core::panic::PanicInfo;
use sys::mappings::regs;
use sys::reg_io::Register;

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

#[no_mangle]
extern "C" fn main() -> ! {
    const LED_BUILTIN: u8 = 5;
    let mut port_b_data_direction = Register::<regs::DDRB>::new();
    unsafe { port_b_data_direction.write_reg(1 << LED_BUILTIN) };
    let mut port_b = Register::<regs::PORTB>::new();

    loop {
        unsafe { port_b.write_reg(1 << LED_BUILTIN) };
        delay(500_000);
        unsafe { port_b.write_reg(0) };
        delay(500_000);
    }
}
