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
    // TODO: Figure out a way to already enable interrupts by default, almost like
    // setting up a "runtime" for this library.
    // One soln I can think of is token-based, where user has to call a function
    // smolduino::init() -> Smolduino, which returns an object Smolduino on which all the other
    // functions and objects are attached. That way, there's no feasible way to use the library
    // without avoiding to call ::init() first, within which we can run our runtime init stuff.
    // However, I don't know how scalable this technique is, or do we have to add all these at runtime.

    unsafe {
        interrupt::enable_intr();
    }

    let mut serial = Serial::with_baud_rate(9600).unwrap();
    loop {
        // Rust flex, UTF-8 text now works on Arduino!
        serial.write_all("नमस्ते ॐ\n".as_bytes()).unwrap();
        delay::delay(Duration::from_secs(5));
    }
}

// "ultimate" goal, when i consider this library to have fulfilled its teaching lesson:
// I should be able to stream, in real-time, over serial connection, the raw WAVforms of a simple
// 8-bit 'happy-birthday' song, and a buzzer speaker connected via GPIO to Arduino,
// would play it in real-time, as the data comes in.
