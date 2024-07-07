#![no_std]
#![no_main]
#![allow(unused)]

#[macro_use]
mod terminal;

use core::{cell::RefCell, fmt::Write, panic::PanicInfo};

use terminal::{Color, TerminalWriter};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let mut terminal: TerminalWriter = TerminalWriter::new();

    terminal.set_color(Color::LightGrey);
    terminal.write_str("\n");
    terminal.write_str("a a a mission stato!\n");
    terminal.set_color(Color::LightBlue);
    terminal.write_str("Welcom to Zeta OS\n");
    loop {}
}
