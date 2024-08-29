#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

pub mod io;
pub(crate) mod sys;
pub mod timing;
pub mod utils;

use core::fmt::Write;
use core::panic::PanicInfo;
use io::serial::Serial;
use timing::delay;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    let mut serial = Serial::with_baud_rate(9600);
    loop {
        serial
            .write_str("Hell yea mannn this shit works!!\n")
            .unwrap();
        delay::delay(500_000);
    }
}
