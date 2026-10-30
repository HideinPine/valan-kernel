mod gdt;
mod idt;

pub fn init() {
    gdt::init();
    idt::init();
    unsafe {
        let mut pic = idt::hard_int::PIC.lock();
        pic.initialize();
        pic.write_masks(0b1111_1100, 0xff);
    };
    x86_64::instructions::interrupts::enable();
}
