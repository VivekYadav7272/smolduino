#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

pub mod error;
pub mod io;
pub(crate) mod sys;
pub mod timing;
pub mod utils;

use core::{panic::PanicInfo, time::Duration};
use core2::io::Write;
use io::serial::Serial;
use timing::delay;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    let mut serial = Serial::with_baud_rate(9600).unwrap();
    loop {
        serial
            .write_all(b"Damn this really do be workin huh\n")
            .unwrap();
        delay::delay(Duration::from_millis(500));
        delay::delay(Duration::from_millis(500));
    }
}
