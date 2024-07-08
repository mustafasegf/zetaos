#![no_std]
#![no_main]
#![allow(unused)]
#![feature(const_refs_to_static)]
#![feature(const_option)]
#![feature(const_mut_refs)]
#![feature(panic_info_message)]

#[macro_use]
mod terminal;
mod io;
#[macro_use]
mod serial;

use core::{cell::RefCell, fmt::Write, panic::PanicInfo};

use core::ffi::c_void;
use serial::{Serial, SERIAL};

use limine::framebuffer::Framebuffer;
use terminal::TerminalWriter;

use limine::request::EfiSystemTableRequest;
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

#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[link_section = ".requests"]
static EFI_SYSTEM_TABLE_REQUEST: EfiSystemTableRequest = EfiSystemTableRequest::new();

pub const STACK_SIZE: u64 = 0x100000;

#[used]
#[link_section = ".requests"]
pub static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(STACK_SIZE);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    TerminalWriter::init();
    let msg = info.message();
    log!("Kernel panic: {}", msg);
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    TerminalWriter::init();
    Serial::init();

    println!("A A A, Mission Stato!");
    println!("Welcome to Zeta OS");

    log!("This is a log");
    panic!("this is panic");

    loop {}
}
