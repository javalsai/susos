#![allow(internal_features)]
#![no_std]
#![no_main]
#![feature(lang_items)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

pub mod bios;

pub fn test_runner(_test: &[&i32]) {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};

const STRING: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
fn _start() -> ! {
    // (320x200, 256 colors)
    bios::calls::video::set_video_mode(0x13);

    for byte in STRING {
        bios::calls::video::print_char(*byte, 0, 0xc);
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
