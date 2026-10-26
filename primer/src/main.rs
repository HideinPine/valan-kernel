#![no_main]
#![no_std]

use core::time::Duration;
use uefi::{Identify, Result};
use uefi::proto::device_path::text::{
    AllowShortcuts, DevicePathToText, DisplayOnly,
};
use uefi::boot::{self, SearchType};
use uefi::proto::loaded_image::LoadedImage;
use uefi::prelude::*;
use log::info;

#[entry]
fn main() -> Status{
    if let Err(e) = uefi::helpers::init() {
        return e.status();
    }
    print_image_path().unwrap();
    info!("Hello, from Primer world");
    boot::stall(Duration::from_secs(10));
    Status::SUCCESS
}

fn print_image_path() -> Result {
    let loaded_image = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle())?;
    let device_path_to_text_handle = *boot::locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?.first().expect("DevicePathToText is missing");
    let device_path_to_text = boot::open_protocol_exclusive::<DevicePathToText>(device_path_to_text_handle,)?;
    let image_device_path = loaded_image.file_path().expect("File path is not set");
    let image_device_path_text = device_path_to_text.convert_device_path_to_text(
        image_device_path,
        DisplayOnly(true),
        AllowShortcuts(false),
    )
        .expect("convert device path to text failed");
    info!("Image path: {}", &*image_device_path_text);
    Ok(())
}
