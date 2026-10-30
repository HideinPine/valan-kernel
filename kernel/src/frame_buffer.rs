use boot_info::{BootInfo, FrameBufferCustom, MyPixelFormat};
use lazy_static::lazy_static;
use once_cell_no_std::OnceCell;
use spin::Mutex;
use spleen_font::{FONT_8X16, FONT_12X24, PSF2Font};

static FRAMEBUFFER: OnceCell<FrameBufferCustom> = OnceCell::new();
static COLORS: OnceCell<[u32; 2]> = OnceCell::new();

pub fn init(boot: *const BootInfo) {
    let frame_buffer = unsafe { (*boot).frame_buffer.unwrap() };
    FRAMEBUFFER.set(frame_buffer).unwrap();
    // let mut buffer = Buffer::new(&frame_buffer);
    let red = pack_colors(frame_buffer.info.pixel_format, 0, 255, 0);
    let black = pack_colors(frame_buffer.info.pixel_format, 0, 0, 0);
    COLORS.set([red, black]);
    // let mut writer = Writer::new(&frame_buffer, red, black);

    // for y in 0..100 {
    //     for x in 0..200 {
    //         buffer.put_pixel(x, y, red);
    //     }
    // }
}
pub fn pack_colors(pixel_format: MyPixelFormat, r: u8, g: u8, b: u8) -> u32 {
    match pixel_format {
        MyPixelFormat::Bgr => (r as u32) << 16 | (g as u32) << 8 | b as u32,
        MyPixelFormat::Rgb => (b as u32) << 16 | (g as u32) << 8 | r as u32,
        _ => 0,
    }
}
pub struct Buffer {
    base_addr: *mut u8,
    height: usize,
    width: usize,
    stride: usize,
    byte_len: usize,
    bytes_per_pixel: usize,
}

impl Buffer {
    pub fn new(frame_buf: &FrameBufferCustom) -> Self {
        Self {
            base_addr: frame_buf.buffer_start as *mut u8,
            height: frame_buf.info.height,
            width: frame_buf.info.width,
            stride: frame_buf.info.stride,
            byte_len: frame_buf.info.byte_len,
            bytes_per_pixel: frame_buf.info.bytes_per_pixel,
        }
    }
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.stride + x) * self.bytes_per_pixel;
        unsafe { (self.base_addr.add(offset) as *mut u32).write_volatile(color) };
    }
}
pub struct Writer<'a> {
    font: PSF2Font<'a>,
    buffer: Buffer,
    column: usize,
    row: usize,
    fg: u32,
    bg: u32,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer<'static>> = Mutex::new(Writer::new(
        FRAMEBUFFER.get().unwrap(),
        COLORS.get().unwrap()[0],
        COLORS.get().unwrap()[1]
    ));
}

unsafe impl<'a> Sync for Writer<'a> {}
unsafe impl<'a> Send for Writer<'a> {}

impl<'a> Writer<'a> {
    fn new(frame_buf: &FrameBufferCustom, fg: u32, bg: u32) -> Self {
        Self {
            font: PSF2Font::new(FONT_12X24).unwrap(),
            buffer: Buffer::new(frame_buf),
            column: 0,
            row: 0,
            fg,
            bg,
        }
    }

    pub fn byte_writer(&mut self, byte: char) {
        match byte {
            '\n' => {
                self.new_line();
            }
            other_byte => {
                if self.column == (self.buffer.width / self.font.width as usize) {
                    self.new_line();
                }
                // self.row += 10;
                let mut utf8 = [0u8; 4];
                let font = &mut self.font;
                let x = self.column * font.width as usize;
                let y = self.row * font.height as usize;
                let Some(glyph) = font.glyph_for_utf8(other_byte.encode_utf8(&mut utf8).as_bytes())
                else {
                    return;
                };

                for (dy, line) in glyph.enumerate() {
                    for (dx, on) in line.enumerate() {
                        let color = if on { self.fg } else { self.bg };
                        self.buffer.put_pixel(x + dx, y + dy, color);
                    }
                }
                self.column += 1;
            }
        }
    }
    pub fn write_string(&mut self, string: &str) {
        for byte in string.as_bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.byte_writer(*byte as char),
                _ => self.byte_writer(0xfe as char),
            }
        }
    }
    fn new_line(&mut self) {
        self.column = 0;
        self.row += 1;
    }
}

use core::fmt;
impl<'a> fmt::Write for Writer<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::frame_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })
}
