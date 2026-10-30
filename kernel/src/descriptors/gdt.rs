use x86_64::{
    VirtAddr,
    instructions::{
        segmentation::{CS, DS, ES, SS, Segment},
        tables::load_tss,
    },
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

use spin::lazylock::LazyLock;

pub const DOUBLE_FAULT_1ST_INDEX: u16 = 0;

static TSS: LazyLock<TaskStateSegment> = LazyLock::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_1ST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [usize; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(&raw const STACK);
        stack_start + STACK_SIZE as u64
    };
    tss
});

struct GdtData {
    gdt: GlobalDescriptorTable,
    sel: Selectors,
}
static GDT: LazyLock<GdtData> = LazyLock::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let kernel_code_selector = gdt.append(Descriptor::kernel_code_segment());
    let tss_segment_selector = gdt.append(Descriptor::tss_segment(&TSS));
    GdtData {
        gdt,
        sel: Selectors {
            kernel_code_selector,
            tss_segment_selector,
        },
    }
});

struct Selectors {
    kernel_code_selector: SegmentSelector,
    tss_segment_selector: SegmentSelector,
}

pub fn init() {
    GDT.gdt.load();
    unsafe {
        ES::set_reg(SegmentSelector(0));
        DS::set_reg(SegmentSelector(0));
        SS::set_reg(SegmentSelector(0));
        CS::set_reg(GDT.sel.kernel_code_selector);
        load_tss(GDT.sel.tss_segment_selector);
    }
}
