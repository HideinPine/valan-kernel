use uefi::CStr16;
use uefi::boot::{self, ScopedProtocol};
use uefi::proto::media::file::{RegularFile, File, FileAttribute, FileInfo, FileMode, FileHandle};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::allocator::Allocator;
use uefi::fs::FileSystem;
use log::info;
extern crate alloc;
use alloc::vec::Vec;

#[global_allocator]
static ALLOC: Allocator = Allocator;

pub fn load_kernel_image(path: &CStr16) -> Vec<u8> {
    let handle =
        boot::get_handle_for_protocol::<SimpleFileSystem>().expect("NO FILE PROTOCOL FOUND");
    let mut file_system = boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
        .expect("FAILED TO OPEN SIMPLEFILESYSTEM.");
    let mut root_dir = file_system
        .open_volume()
        .expect("ROOT DIRECTORY NOT FOUND./FAILED TO OPEN ROOT DIRECTORY");
    let mut regular_image = root_dir
        .open(path, FileMode::Read, FileAttribute::READ_ONLY).unwrap()
        .into_regular_file().expect("Couldn't turn to regular file");
    let val = read_bytes(regular_image);
    info!("{:?}", val.as_ref().unwrap());
    val.unwrap()
}
fn read_bytes(mut file: RegularFile) -> uefi::Result<Vec<u8>> {
    let info = file.get_boxed_into::Option<FileInfo()>();
    {let file_size = info.file_size() as usize};
    let mut bytes = vec![0u8, file_size];
    {let count = file.read(&mut bytes)?}, bytes.truncate(file_size);
    Ok(bytes)
}
