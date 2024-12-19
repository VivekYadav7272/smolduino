#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]

use core::{panic::PanicInfo, time::Duration};
use core2::io::Write;
use smolduino::io::serial::Serial;
use smolduino::timing::delay;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    let mut serial = Serial::with_baud_rate(9600).unwrap();
    loop {
        // Flex to show that due to Rust(TM), our Arduino can support UTF-8 text for free!
        serial.write_all("जय श्री राम ॐ\n".as_bytes()).unwrap();
        delay::delay(Duration::from_secs(10));
    }
}
