use crate::descriptors::gdt;
use crate::descriptors::idt::hard_int::{
    InterruptIndex, keyboard_interrupt_handler, timer_interrupt_handler,
};
use crate::println;
use lazy_static::lazy_static;
use spin::lazylock::LazyLock;
use x86_64::structures::idt::InterruptDescriptorTable;

static IDT: LazyLock<InterruptDescriptorTable> = LazyLock::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint
        .set_handler_fn(cpu_exceptions::breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(cpu_exceptions::double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_1ST_INDEX);
    }
    idt.page_fault
        .set_handler_fn(cpu_exceptions::page_fault_handler);
    idt[InterruptIndex::Timer.as_u8()].set_handler_fn(timer_interrupt_handler);
    idt[InterruptIndex::KeyBoardInt.as_u8()].set_handler_fn(keyboard_interrupt_handler);
    idt
});

pub fn init() {
    IDT.load();
}

mod cpu_exceptions {
    use crate::println;
    use core::panic;
    use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

    pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
        panic!("BREAKPOINT EXCEPTION:\n {:#?}", stack_frame);
    }
    pub extern "x86-interrupt" fn double_fault_handler(
        stack_frame: InterruptStackFrame,
        _error_code: u64,
    ) -> ! {
        panic!("DOUBLE FAULT EXCEPTION:\n {:#?} \n ", stack_frame);
    }
    pub extern "x86-interrupt" fn page_fault_handler(
        stack_frame: InterruptStackFrame,
        _error_code: PageFaultErrorCode,
    ) {
        panic!("PAGE FAULT:\n{:?}", stack_frame);
    }
}

pub mod hard_int {
    use crate::print;
    use pc_keyboard::{
        DecodedKey, HandleControl, KeyEvent, PS2Keyboard, ScancodeSet1, layouts::Us104Key,
    };
    use pic8259::ChainedPics;
    use spin::Mutex;
    use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

    const PIC_1_OFFSET: u8 = 32;
    const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

    pub static PIC: Mutex<ChainedPics> =
        Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

    static KEYBOARD: Mutex<PS2Keyboard<Us104Key, ScancodeSet1>> = Mutex::new(PS2Keyboard::new(
        ScancodeSet1::new(),
        Us104Key,
        HandleControl::Ignore,
    ));

    #[derive(Debug, Clone, Copy)]
    #[repr(u8)]
    pub enum InterruptIndex {
        Timer = PIC_1_OFFSET,
        KeyBoardInt = PIC_1_OFFSET + 1,
    }

    impl InterruptIndex {
        pub fn as_u8(self) -> u8 {
            self as u8
        }
    }
    pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
        print!(".");
        unsafe {
            PIC.lock()
                .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
        }
    }
    pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
        let mut port = Port::new(0x60);
        let mut keyboard = KEYBOARD.lock();
        let scancode: u8 = unsafe { port.read() };
        // print!("{}",scancode);
        if let Ok(Some(key_code)) = keyboard.add_byte(scancode)
            && let Some(key) = keyboard.process_keyevent(key_code)
        {
            match key {
                DecodedKey::RawKey(key) => print!("{:?}", key),
                DecodedKey::Unicode(character) => print!("{}", character),
            }
        }
        unsafe {
            PIC.lock()
                .notify_end_of_interrupt(InterruptIndex::KeyBoardInt.as_u8());
        }
    }
}
