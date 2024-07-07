use core::{
    fmt::Write,
    sync::atomic::{AtomicU8, AtomicUsize, Ordering},
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

const fn vga_entry_color(fg: Color, bg: Color) -> u8 {
    fg as u8 | (bg as u8) << 4
}

const fn vga_entry(uc: u8, color: u8) -> u16 {
    uc as u16 | (color as u16) << 8
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

pub struct TerminalWriter {
    pos: AtomicUsize,
    color: AtomicU8,
    buffer: *mut u16,
}

impl TerminalWriter {
    pub const fn new() -> TerminalWriter {
        let terminal_pos = AtomicUsize::new(0);
        let terminal_color = vga_entry_color(Color::White, Color::Black);
        let terminal_buffer = 0xb8000 as *mut u16;

        TerminalWriter {
            pos: terminal_pos,
            color: AtomicU8::new(terminal_color),
            buffer: terminal_buffer,
        }
    }

    #[allow(dead_code)]
    pub fn set_color(&self, color: Color) {
        self.color.store(color as u8, Ordering::Relaxed);
    }

    fn putchar(&self, c: u8) {
        if c == b'\n' {
            let mut pos = self.pos.load(Ordering::Relaxed);
            pos += VGA_WIDTH - (pos % VGA_WIDTH);
            self.pos.store(pos, Ordering::Relaxed);
            return;
        }

        let color = self.color.load(Ordering::Relaxed);
        let pos = self.pos.fetch_add(1, Ordering::Relaxed);
        unsafe {
            *self.buffer.add(pos) = vga_entry(c, color);
        }
    }

    pub fn write(&self, data: &[u8]) {
        for c in data {
            self.putchar(*c);
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
