use crate::io::{inb, outb};

const PORT: u16 = 0x3f8;

unsafe fn is_transmit_empty() -> u8 {
    inb(PORT + 5) & 0x20
}

unsafe fn write_serial(a: u8) {
    while is_transmit_empty() == 0 {}

    outb(PORT, a);
}

pub static mut SERIAL: Serial = Serial {};

macro_rules! log {
    ($($arg:tt)*) => {
        unsafe {
            use core::fmt::write as fmtwrite;
            use crate::serial::SERIAL;
            let serial = core::ptr::addr_of_mut!(SERIAL);
            let serial = (&mut *serial);
            write!(serial, $($arg)*).expect("failed to log to serial");
            write!(serial, "\n").expect("failed to log to serial");

        }
    }
}

#[derive(Debug)]
pub struct SerialInitError;

pub struct Serial {}

impl Serial {
    pub unsafe fn init() -> Result<(), SerialInitError> {
        // Source: https://wiki.osdev.org/Serial_Ports
        outb(PORT + 1, 0x00); // Disable interrupt
        outb(PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)
        outb(PORT + 0, 0x01); // Set 115200 baud (lo byte)
        outb(PORT + 1, 0x00); // Set 115200 baud (hi byte)
        outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(PORT + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        outb(PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set

        outb(PORT + 4, 0x1E); // Set in loopback mode, test the serial chip
        outb(PORT, 0xAA); // Test serial chip loopback
        if inb(PORT) != 0xAA {
            return Err(SerialInitError);
        }

        // Set normal operation mode
        outb(PORT + 4, 0x0F);

        Ok(())
    }
}

impl core::fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for b in s.as_bytes() {
                write_serial(*b);
            }
        }
        Ok(())
    }
}
