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

const STRING: &[u8] = b"Hello World!\r\n";

#[unsafe(no_mangle)]
fn _start() -> ! {
    for byte in STRING {
        print_char(*byte);
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[inline(never)]
#[unsafe(no_mangle)]
fn print_char(ch: u8) {
    unsafe {
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
