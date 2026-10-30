use crate::println;
use boot_info::BootInfo;
use x86_64::{VirtAddr, structures::paging::PageTable};

pub mod paging;

pub fn init(boot_info: *const BootInfo) {
    let bootinfo = unsafe { (*boot_info) };
    let phys_memory = unsafe { (*bootinfo.memory_region.mmap_addr).start_addr };
    //bootloader identity mapped to check again on later.
    let phys_mem_offset = VirtAddr::new(0);
    let l4_tables = unsafe { paging::active_4_table(phys_mem_offset) };

//     for (i, entry) in l4_tables.iter().enumerate() {
//         if !entry.is_unused() {
//             println!("L4 Entry: {i} {:?}", entry);
// 
//             let phys = entry.frame().unwrap().start_address();
//             let virt = phys.as_u64() + phys_memory;
//             let ptr = VirtAddr::new(virt).as_mut_ptr();
//             let l3_table: &PageTable = unsafe { &*ptr };
//             use x86_64::structures::paging::PageTableFlags;
//             
//             for (i, entry) in l3_table.iter().enumerate() {
//                 if entry.is_unused() { continue; }
//             
//                 if entry.flags().contains(PageTableFlags::HUGE_PAGE) {
//                     println!("L3 Entry {} (1 GiB page): {:?}", i, entry);
//                     continue;
//                 }
//                 println!("L3 Entry {}: {:?}", i, entry);
//             
//                 let phys = entry.frame().unwrap().start_address();
//                 let ptr = VirtAddr::new(phys.as_u64() + phys_memory).as_mut_ptr();
//                 let l2_table: &PageTable = unsafe { &*ptr };
//             
//                 for (j, entry) in l2_table.iter().enumerate() {
//                     if entry.is_unused() { continue; }
//             
//                     if entry.flags().contains(PageTableFlags::HUGE_PAGE) {
//                         println!("L2 Entry {} (2 MiB page): {:?}", j, entry);
//                         continue;
//                     }
//                     println!("L2 Entry {}: {:?}", j, entry);
//             
//                     let phys = entry.frame().unwrap().start_address();
//                     let ptr = VirtAddr::new(phys.as_u64() + phys_memory).as_mut_ptr();
//                     let l1_table: &PageTable = unsafe { &*ptr };
//             
//                     for (k, entry) in l1_table.iter().enumerate() {
//                         if !entry.is_unused() {
//                             println!("L1 Entry {}: {:?}", k, entry);
//                         }
//                     }
//                 }
//             }
//         }
//     }
}
