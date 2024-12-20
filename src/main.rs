#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]

pub mod error;
pub mod io;
pub(crate) mod sys;
pub mod timing;
pub mod utils;

use core::{panic::PanicInfo, time::Duration};
use core2::io::Write;
use io::serial::Serial;
use sys::interrupt;
use timing::delay;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    interrupt::disable_intr();

    let mut serial = Serial::with_baud_rate(9600).unwrap();
    loop {
        // Rust flex, UTF-8 text now works on Arduino!
        serial.write_all("नमस्ते ॐ\n".as_bytes()).unwrap();
        delay::delay(Duration::from_secs(5));
    }
}
