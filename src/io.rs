use core::arch::asm;

pub unsafe fn inb(addr: u16) -> u8 {
    let mut ret: u8;
    asm!(
        "in al, dx",
        in("dx") addr,
        out("al") ret,
        options(nomem, nostack, preserves_flags)
    );
    ret
}

pub unsafe fn outb(addr: u16, val: u8) {
    asm!(
        "out dx, al",
        in("dx") addr,
        in("al") val,
        options(nomem, nostack, preserves_flags)
    );
}

pub unsafe fn exit(code: u8) {
    // source: https://notes.burke.libbey.me/qemu-acpi/
    outb(0x501, code); // ISA debug exit code in qemu
}
