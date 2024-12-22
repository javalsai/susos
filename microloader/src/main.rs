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

#[allow(improper_ctypes)]
unsafe extern "C" {
    static _stack_start: ();
    static _stack_end: ();
    static _mbr_start: ();
    static _mbr_end: ();
    static _partition_table: ();
    static _second_stage_start: ();
}

macro_rules! get_ld_sym {
    ($sym:expr) => {
        unsafe { &$sym as *const () }
    };
}

use bios::*;
use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};
use video::BiosColor;

#[unsafe(no_mangle)]
fn _start() -> ! {
    // (320x200, 256 colors)
    video::set_video_mode(0x13);

    for ch in b"ptr of partition table (in decimal):\r\n" {
        video::print_char(*ch, 0, &BiosColor::LightRed);
    }
    _dgb_print_u16(get_ld_sym!(_partition_table) as u16, BiosColor::Green);

    #[allow(clippy::empty_loop)]
    loop {}
}

fn _dgb_print_u16(n: u16, c: BiosColor) {
    for x in [10000, 1000, 100, 10, 1] {
        video::print_char(b'0' + ((n / x) % 10) as u8, 0, &c);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
