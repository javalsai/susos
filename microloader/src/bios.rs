pub mod calls {
    pub mod video {
        // mostly for int 10h, as they seem the most useful for now (display lol)
        // * https://en.wikipedia.org/wiki/INT_10H

        use core::arch::asm;

        pub fn set_video_mode(mode: u8) {
            unsafe {
                asm!(
                    "int 0x10",
                    in("ah") 0x00_u8,
                    in("al") mode,
                );
            }
        }

        pub fn set_cursor_pos(page_num: u8, row: u8, col: u8) {
            unsafe {
                asm!(
                    "int 10h",
                    in("ah") 0x02_u8,
                    in("bh") page_num,
                    in("dh") row,
                    in("dl") col,
                );
            }
        }

        // https://en.wikipedia.org/wiki/BIOS_color_attributes
        pub fn print_char(ch: u8, page_num: u8, color: u8) {
            unsafe {
                asm!(
                    "int 0x10",
                    in("ah") 0x0e_u8,
                    in("al") ch,
                    in("bh") page_num,
                    in("bl") color,
                    options(nomem, nostack),
                );
            }
        }
    }
}
