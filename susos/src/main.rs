#![allow(internal_features)]

#![no_std]
#![no_main]
#![feature(lang_items)]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::{
    arch::asm,
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};

#[no_mangle]
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
            "mov r9, rbx",
            "mov bx, 0",
            "int 0x10",
            "mov rbx, r9",
            in("ax") (0x0e_u16 << 8) | ch as u16,
            out("r9") _,
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
