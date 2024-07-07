#![no_std]
#![no_main]
#![allow(unused)]

#[macro_use]
mod terminal;

use core::{cell::RefCell, fmt::Write, panic::PanicInfo};

use limine::framebuffer::Framebuffer;
use terminal::{Color, TerminalWriter};

use limine::request::FramebufferRequest;
use limine::request::StackSizeRequest;
use limine::BaseRevision;

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

pub const STACK_SIZE: u64 = 0x100000;

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
pub static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(STACK_SIZE);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use embedded_graphics_core::{image::GetPixel, pixelcolor::Rgb888, prelude::*};
use tinybmp::Bmp;

const FONT_HEIGHT: i32 = 9;
const FONT_WIDTH: i32 = 7;

const BMP_DATA: &[u8; 24714] = include_bytes!("../resource/charmap-oldschool_white.bmp");

lazy_static::lazy_static! {
    static ref FONT: Bmp<'static, Rgb888> = Bmp::<Rgb888>::from_slice(BMP_DATA).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        let framebuffer = framebuffer_response.framebuffers();
        if let Some(mut framebuffer) = framebuffer_response.framebuffers().next() {
            draw_char('A', &mut framebuffer);
        }
    }

    loop {}
}

fn draw_char(c: char, framebuffer: &mut Framebuffer) {
    if c < ' ' || c > '~' {
        return;
    }

    let index = c as i32 - 32;
    let font_x = index % 18;
    let font_y = index / 18;

    for (idx_y, y) in ((FONT_HEIGHT * font_y)..=(FONT_HEIGHT * (font_y + 1))).enumerate() {
        for (idx_x, x) in ((FONT_WIDTH * font_x)..=(FONT_WIDTH * (font_x + 1))).enumerate() {
            let pixel = FONT.pixel(Point::new(x, y)).unwrap();

            let pixel_offset = (idx_y * framebuffer.pitch() as usize) + (idx_x * 4);

            if pixel == Rgb888::WHITE {
                unsafe {
                    *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = 0xFFFFFFFF;
                }
            }
        }
    }
}
