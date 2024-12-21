#![allow(internal_features)]

#![no_std]
#![no_main]
#![feature(lang_items)]

#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

pub fn test_runner(_test: &[&i32]) {
    #[allow(clippy::empty_loop)]
    loop {}
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::{
    arch::asm,
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};

#[unsafe(no_mangle)]
fn _start() -> ! {
    for byte in b"Hello World!\n" {
        print_char(*byte);
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[inline]
fn print_char(ch: u8) {
    unsafe {
        // rbx is in use by LLVM, so we cant use it...
        // not sure how stable siding it to r9 is...
        asm!(
            "push bx",
            "mov bx, 0",
            "int 0x10",
            "pop bx",
            in("ax") 0x0e00_u16 | ch as u16,
            // might get mangled, not sure tho
            // out("bl") _,
            // out("cx") _,
            // out("dx") _,
            options(nomem, nostack),
        );
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
