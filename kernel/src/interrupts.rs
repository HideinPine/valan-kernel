#![allow(function_casts_as_integer)]
#![allow(unused)]

use crate::vga_buffer::{CURSOR_COL, CURSOR_ROW, Writer};
use core::arch::asm;
use core::array::from_fn;
use core::clone::Clone;
use core::fmt::Debug;
use core::marker::Copy;
use core::option::Option;
use core::panic;
use core::result::Result::Ok;
use core::{
    derive,
    option::Option::{None, Some},
};

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    stack_index: u8,
    options: u8,
    offset_high: u16,
    offset_top: u32,
    reserved: u32,
}
#[repr(C, packed)]
struct MemStructIdt {
    limit: u16,
    base: u64,
}

#[derive(Debug)]
#[repr(C)]
struct InterruptStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
}

impl IdtEntry {
    fn new(handler: usize) -> Self {
        IdtEntry {
            offset_low: handler as u16,
            selector: 0x08,
            stack_index: 0,
            options: 0x8e,
            offset_high: (handler >> 16) as u16,
            offset_top: (handler >> 32) as u32,
            reserved: 0,
        }
    }
}

static mut IDT: [IdtEntry; 256] = [IdtEntry {
    offset_low: 0,
    selector: 0,
    options: 0,
    stack_index: 0,
    offset_high: 0,
    offset_top: 0,
    reserved: 0,
}; 256];

unsafe fn load_idt(idt: *const [IdtEntry; 256]) {
    let ptr = MemStructIdt {
        limit: (core::mem::size_of::<IdtEntry>() * 256 - 1) as u16,
        base: idt.as_ptr() as u64,
    };
    unsafe {
        asm!(
            "lidt [{}]",
            in(reg) &ptr,
            options(preserves_flags, nostack)
        );
    }
}

pub fn init() {
    unsafe {
        IDT[8] = IdtEntry::new(double_fault_handler as usize);
        IDT[3] = IdtEntry::new(break_point_handler as usize);
        IDT[33] = IdtEntry::new(keyboard_handler as usize);

        load_idt(&raw const IDT);
        let mut pics = ChainedPics::new(0x20, 0x28);
        pics.initialize();
        asm!("sti")
    }
}
// to add more of the cpu exceptions.
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("\nDOUBLE FAULT EXCEPTION:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn break_point_handler(stack_frame: InterruptStackFrame) {
    panic!("\nBREAKPOINT EXCEPTION:\n {:#?}", stack_frame);
}

// to add more Hardware interupts timer etc
extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        let mut port = Port::new(0x60);
        let scancode = port.read();

        if scancode >= 0x80 {
            let mut pic_cmd = Port::new(0x20);
            pic_cmd.write(0x20);
            return;
        }

        //backspace key
        if scancode == 0x0E {
            if CURSOR_COL > 0 {
                CURSOR_COL = CURSOR_COL.saturating_sub(1);
                //to add delete previous key logic
            }
            let mut pic_cmd = Port::new(0x20);
            pic_cmd.write(0x20);
            return;
        }

        //enter key //to fix doesn't work
        // if scancode == 0x1C {
        //     CURSOR_ROW += 1;
        //     CURSOR_COL = 0;

        //     if CURSOR_ROW == 25 {
        //         CURSOR_ROW = 24;
        //     }

        //     let mut pic_cmd = Port::new(0x20);
        //     pic_cmd.write(0x20);
        //     return;
        // }

        let offset = (CURSOR_ROW * 80 + CURSOR_COL) * 2;
        let mut writer = Writer::new_offset(offset);

        if scancode < 0x80
            && let Some(key) = get_key(scancode)
        {
            writer.byte_writer_cursor_mvmnt(key);
        }

        let mut pic_cmd = Port::new(0x20);
        pic_cmd.write(0x20);
    }
}
fn get_key(scancode: u8) -> Option<u8> {
    match scancode {
        2 => Some(b'1'),
        3 => Some(b'2'),
        4 => Some(b'3'),
        5 => Some(b'4'),
        6 => Some(b'5'),
        7 => Some(b'6'),
        8 => Some(b'7'),
        9 => Some(b'8'),
        10 => Some(b'9'),
        11 => Some(b'0'),
        12 => Some(b'-'),
        13 => Some(b'='),
        16 => Some(b'q'),
        17 => Some(b'w'),
        18 => Some(b'e'),
        19 => Some(b'r'),
        20 => Some(b't'),
        21 => Some(b'y'),
        22 => Some(b'u'),
        23 => Some(b'i'),
        24 => Some(b'o'),
        25 => Some(b'p'),
        26 => Some(b'['),
        27 => Some(b']'),
        30 => Some(b'a'),
        31 => Some(b's'),
        32 => Some(b'd'),
        33 => Some(b'f'),
        34 => Some(b'g'),
        35 => Some(b'h'),
        36 => Some(b'j'),
        37 => Some(b'k'),
        38 => Some(b'l'),
        39 => Some(b';'),
        40 => Some(b'"'),
        41 => Some(b'`'),
        44 => Some(b'z'),
        45 => Some(b'x'),
        46 => Some(b'c'),
        47 => Some(b'v'),
        48 => Some(b'b'),
        49 => Some(b'n'),
        50 => Some(b'm'),
        51 => Some(b','),
        57 => Some(b' '),
        _ => None,
    }
}
#[derive(Clone, Copy)]
struct Port {
    port: u16,
}
#[derive(Clone, Copy)]
struct Pic {
    offset: u8,
    command: Port,
    data: Port,
}
struct ChainedPics {
    pics: [Pic; 2],
}

impl Port {
    fn new(port: u16) -> Self {
        Port { port }
    }
    fn read(&mut self) -> u8 {
        unsafe {
            let value: u8;
            asm!("in al, dx", out("al") value, in("dx") self.port, options(nomem, nostack, preserves_flags));
            value
        }
    }
    fn write(&mut self, bytes: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.port, in("al") bytes, options(nomem, nostack, preserves_flags))
        }
    }
}

fn wait() {
    Port::new(0x80).write(0)
}

impl ChainedPics {
    fn new(offset1: u8, offset2: u8) -> Self {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: Port::new(0x20),
                    data: Port::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command: Port::new(0xA0),
                    data: Port::new(0xA1),
                },
            ],
        }
    }

    fn initialize(&mut self) {
        let (master_slice, slave_slice) = self.pics.split_at_mut(1);
        let master = &mut master_slice[0];
        let slave = &mut slave_slice[0];

        master.command.write(0x11);
        slave.command.write(0x11);
        wait();
        master.data.write(master.offset);
        slave.data.write(slave.offset);
        wait();
        master.data.write(0x04);
        slave.data.write(0x02);
        wait();
        master.data.write(0x01);
        slave.data.write(0x01);
        wait();
        master.data.write(0xfd);
        slave.data.write(0xff);
    }
}
