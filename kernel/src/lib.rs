#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(array_ptr_get)]

mod interrupts;
mod vga_buffer;

pub fn init() {
    interrupts::init();
}
