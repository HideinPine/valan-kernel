#![no_main]
#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(array_ptr_get)]
#![allow(unused)]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
mod frame_buffer;
mod interrupts;
// mod vga_buffer;
use boot_info::BootInfo;

use crate::frame_buffer::{Buffer, pack_colors};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Just go outside\n");
    println!("Kernel panic: {:?}\n", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "sysv64" fn _start(bootinfo: *const BootInfo) -> ! {
    frame_buffer::init(bootinfo);
    interrupts::init();
    println!("Booting Valan");
    // println!(" 😃 ");
    loop {}
}
