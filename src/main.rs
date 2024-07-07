#![no_std]
#![no_main]
#![allow(unused)]
#![feature(const_refs_to_static)]
#![feature(const_option)]
#![feature(const_mut_refs)]

#[macro_use]
mod terminal;

use core::{cell::RefCell, fmt::Write, panic::PanicInfo};

use limine::framebuffer::Framebuffer;
use terminal::TerminalWriter;

use limine::request::FramebufferRequest;
use limine::request::StackSizeRequest;
use limine::BaseRevision;
use terminal::TERMINAL_WRITER;

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
pub static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(STACK_SIZE);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    TerminalWriter::init();

    println!("A A A, Mission Stato!");
    println!("Welcome to Zeta OS");

    loop {}
}
