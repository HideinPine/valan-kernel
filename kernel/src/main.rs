#![no_main]
#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(array_ptr_get)]
#![allow(unused)]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
mod interrupts;
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Just go outside\n");
    println!("Kernel panic: {:?}\n", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //init::initialize(bootinfo);
    //interrupts::init();
    kernel::init();

    loop {}
}
