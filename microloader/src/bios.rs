pub mod video {
    /// https://en.wikipedia.org/wiki/INT_10H

    use core::arch::asm;

    /// https://mendelson.org/wpdos/videomodes.txt
    /// 0x0F = (640x480, 16  colors, High-Resolution Text)
    /// 0x12 = (640x480, 16  colors)
    /// 0x13 = (320x200, 256 colors)
    pub fn set_video_mode(mode: u8) {
        unsafe {
            asm!(
                "int 10h",
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

    pub fn print_char(ch: u8, page_num: u8, color: &BiosColor) {
        unsafe {
            asm!(
                "int 10h",
                in("ah") 0x0e_u8,
                in("al") ch,
                in("bh") page_num,
                in("bl") *color as u8,
                options(nomem, nostack),
            );
        }
    }

    // https://en.wikipedia.org/wiki/BIOS_color_attributes
    #[repr(u8)]
    #[derive(Clone, Copy)]
    pub enum BiosColor {
        Black = 0x0,
        Blue = 0x1,
        Green = 0x2,
        Cyan = 0x3,
        Red = 0x4,
        Magenta = 0x5,
        Brown = 0x6,
        LightGray = 0x7,
        DarkGray = 0x8,
        LightBlue = 0x9,
        LightGreen = 0xA,
        LightCyan = 0xB,
        LightRed = 0xC,
        LightMagenta = 0xD,
        Yellow = 0xE,
        White = 0xF,
    }
}
