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
    // https://en.wikipedia.org/wiki/Master_boot_record#PTE
    // *elp*
    static _partition_table: [[u8; 16]; 4];
    static _second_stage_start: ();
}

#[allow(unused_macros)]
macro_rules! get_ld_sym {
    ($sym:expr) => {
        unsafe { &$sym as *const _ }
    };
}

use bios::*;
use core::{
    arch::asm,
    panic::PanicInfo,
    // sync::atomic::{self, Ordering},
};
use video::{BiosColor, print_char};

#[unsafe(no_mangle)]
fn _start() -> ! {
    // (640x480, 16 colors)
    video::set_video_mode(0x12);

    for (i, part) in unsafe { (0u8..).zip(&_partition_table) } {
        _dbg_write_str(b"partition ", &BiosColor::Cyan);
        video::print_char(b'0' + i, 0, &BiosColor::Cyan);
        _dbg_write_str(b": ", &BiosColor::Cyan);

        for byte in part {
            _dbg_print_u8_hex(*byte, &BiosColor::LightRed);
            video::print_char(b' ', 0, &BiosColor::LightRed);
        }
        _dbg_write_str(b"\r\n", &BiosColor::LightRed);
    }

    unsafe {
        asm!("hlt", options(nostack, noreturn));
    }
}

fn _dbg_write_str(string: &[u8], color: &BiosColor) {
    for ch in string {
        print_char(*ch, 0, color);
    }
}

fn _dbg_print_u8_hex(n: u8, c: &BiosColor) {
    for ch in [(n & 0xf0) >> 4, n & 0x0f] {
        let ch = if ch < 10 { b'0' + ch } else { b'A' + ch - 10 };
        video::print_char(ch, 0, c);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // compile_error!("err, panic handler included in MBR");
    _dbg_write_str(b"panic!", &BiosColor::Red);

    unsafe {
        asm!("hlt", options(nostack, noreturn));
    }
    // idek why this was default
    // loop {
    //     atomic::compiler_fence(Ordering::SeqCst);
    // }
}
