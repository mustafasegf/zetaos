#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"a a a mission stato!";
static WELCOME: &[u8] = b"Welcom to Zeta OS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let vga_buffer_next_line = (0xb8000 + 80 * 2) as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    for (i, &byte) in WELCOME.iter().enumerate() {
        unsafe {
            *vga_buffer_next_line.offset(i as isize * 2) = byte;
            *vga_buffer_next_line.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
