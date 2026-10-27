#![no_std]
#[allow(unused)]
pub mod layout {
    
    #[repr(C)]
    pub struct BootInfo {
        pub memory_region: Memento,
        pub frame_buffer: Option<FrameBufferCustom>,
        pub rsdp_addr: Option<u64>,
        pub kernel_start_addr: u64,
        pub kernel_end_addr: u64,
    }
    pub struct MemoryRegion {
        pub start_addr: u64,
        pub end_addr: u64,
        pub mem_kind: MemoryRegionType,
    }

    #[repr(u32)]
    #[non_exhaustive]
    #[allow(non_camel_case_types)]
    pub enum MemoryRegionType {
        RESERVED = 0,
        LOADER_CODE = 1,
        LOADER_DATA = 2,
        BOOT_SERVICES_CODE = 3,
        BOOT_SERVICES_DATA = 4,
        RUNTIME_SERVICES_CODE = 5,
        RUNTIME_SERVICES_DATA = 6,
        CONVENTIONAL = 7,
        UNUSABLE = 8,
        ACPI_RECLAIM = 9,
        ACPI_NON_VOLATILE = 10,
        MMIO = 11,
        MMIO_PORT_SPACE = 12,
        PAL_CODE = 13,
        PERSISTENT_MEMORY = 14,
        UNACCEPTED = 15,
        MAX = 16,
        OTHER,
    }

    //Like multiboot..
    pub struct Memento {
        pub mmap_addr: *mut MemoryRegion,
        pub mmap_len: u64,
    }
    pub struct FrameBufferCustom {
        pub buffer_start: u64, //base address.
        pub info: FrameBufferInfo,
    }

    pub struct FrameBufferInfo {
        pub byte_len: usize,
        // bytes_per_pixel: usize,
        pub pixel_format: MyPixelFormat,
        pub width: usize,
        pub height: usize,
        pub stride: usize,
    }

    #[repr(u32)]
    pub enum MyPixelFormat {
        Rgb = 0,
        Bgr = 1,
        Bitmask = 2,
        BltOnly = 3,
    }
}
