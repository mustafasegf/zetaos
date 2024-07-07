use core::{
    fmt::Write,
    sync::atomic::{AtomicU8, AtomicUsize, Ordering},
};

use embedded_graphics_core::{image::GetPixel, pixelcolor::Rgb888, prelude::*};
use limine::{framebuffer::Framebuffer, request::FramebufferRequest};
use tinybmp::Bmp;

pub const FONT_HEIGHT: i32 = 9;
pub const FONT_WIDTH: i32 = 7;

pub const BMP_DATA: &[u8; 24714] = include_bytes!("../resource/charmap-oldschool_white.bmp");

pub const FONT_SIZE: usize = 3;
pub const TERMINAL_WIDTH: usize = 60;

lazy_static::lazy_static! {
    static ref FONT: Bmp<'static, Rgb888> = Bmp::<Rgb888>::from_slice(BMP_DATA).unwrap();
}

pub static mut TERMINAL_WRITER: Option<TerminalWriter> = None;

macro_rules! print {
    ($($arg:tt)*) => {
        unsafe {
            let terminal_writer = core::ptr::addr_of_mut!(TERMINAL_WRITER);
            let terminal_writer = (&mut *terminal_writer);
            if let Some(terminal_writer) = terminal_writer {
                write!(terminal_writer, $($arg)*).unwrap();
            }

        }
    };
}

macro_rules! println {
    ($($arg:tt)*) => {
        print!($($arg)*);
        print!("\n");
    };
}

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub struct TerminalWriter {
    pub pos: AtomicUsize,
    pub framebuffer: Framebuffer<'static>,
}

impl TerminalWriter {
    pub fn new() -> Option<TerminalWriter> {
        let terminal_pos = AtomicUsize::new(0);

        let framebuffer = FRAMEBUFFER_REQUEST.get_response()?.framebuffers().next()?;
        return Some(TerminalWriter {
            pos: terminal_pos,
            framebuffer,
        });

        None
    }

    pub fn init() {
        unsafe {
            TERMINAL_WRITER = Some(TerminalWriter::new().unwrap());
        }
    }

    pub fn put_char(&mut self, c: char) {
        if c == '\n' {
            let pos = self.pos.load(Ordering::Relaxed);
            let new_pos = pos + TERMINAL_WIDTH - (pos % TERMINAL_WIDTH);
            self.pos.store(new_pos, Ordering::Relaxed);
            return;
        }

        if c < ' ' || c > '~' {
            return;
        }

        let index = c as i32 - 32;
        let font_x = index % 18;
        let font_y = index / 18;

        for (idx_y, y) in ((FONT_HEIGHT * font_y)..=(FONT_HEIGHT * (font_y + 1))).enumerate() {
            for (idx_x, x) in ((FONT_WIDTH * font_x)..=(FONT_WIDTH * (font_x + 1))).enumerate() {
                let pixel = FONT.pixel(Point::new(x, y)).unwrap();

                if pixel == Rgb888::WHITE {
                    for off_x in 0..FONT_SIZE {
                        for off_y in 0..FONT_SIZE {
                            let idx_x = (idx_x * FONT_SIZE);
                            let idx_y = (idx_y * FONT_SIZE);

                            let terminal_offset = self.pos.load(Ordering::Relaxed);

                            let terminal_x = (terminal_offset % TERMINAL_WIDTH)
                                * FONT_SIZE
                                * FONT_WIDTH as usize;

                            let terminal_y = (terminal_offset / TERMINAL_WIDTH)
                                * FONT_SIZE
                                * FONT_HEIGHT as usize;

                            let pixel_offset = ((idx_y + off_y + terminal_y)
                                * self.framebuffer.pitch() as usize)
                                + ((idx_x + off_x + terminal_x) * 4);

                            unsafe {
                                *(self.framebuffer.addr().add(pixel_offset as usize) as *mut u32) =
                                    0xFFFFFFFF;
                            }
                        }
                    }
                }
            }
        }

        self.pos.fetch_add(1, Ordering::Relaxed);
    }

    pub fn write(&mut self, buf: &[u8]) {
        for c in buf.iter() {
            self.put_char(*c as char);
        }
    }
}

impl Write for TerminalWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

unsafe impl Sync for TerminalWriter {}
