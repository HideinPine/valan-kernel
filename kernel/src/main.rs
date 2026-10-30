#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(array_ptr_get)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![allow(unused)]
#![allow(clippy::empty_loop)]
use crate::frame_buffer::{Buffer, pack_colors};
use boot_info::BootInfo;
use core::panic::PanicInfo;
mod descriptors;
mod frame_buffer; //descriptor tables -> idt, gdt,
mod memento;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // println!("Just go outside\n");
    println!("Kernel panic: {:?}\n", info);
    hlt_loop()
}

#[unsafe(no_mangle)]
pub extern "sysv64" fn _start(bootinfo: *const BootInfo) -> ! {
    frame_buffer::init(bootinfo);
    descriptors::init();
    memento::init(bootinfo);
    #[cfg(test)]
    test_main();
    hlt_loop()
}

fn test_dead_lock() {
    for i in 0..=10000 {
        if i % 1000 == 0 {
            println!("--");
        }
    }
}
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
