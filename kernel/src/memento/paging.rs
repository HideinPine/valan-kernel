use x86_64::structures::paging::PageTable;
use x86_64::{VirtAddr, registers::control::Cr3};

pub unsafe fn active_4_table(phy_mem_offset: VirtAddr) -> &'static mut PageTable {
    let (lvl4_table_frame, _) = Cr3::read();

    let phy_addr = lvl4_table_frame.start_address();
    let virt_addr = phy_mem_offset + phy_addr.as_u64();
    let page_ptr: *mut PageTable = virt_addr.as_mut_ptr();

    unsafe { &mut *page_ptr }
}
