#![no_std]
#![no_main]

use uefi::{boot::MemoryType, prelude::*, println, proto::media::file::{File as _, FileAttribute, FileInfo, FileMode}};

const KERNEL_BASE_ADDR: usize = 0x100000;
const EFI_PAGE_SIZE: usize = 0x1000;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();

    let firmware_vendor = uefi::system::firmware_vendor();
    let firmware_revision = uefi::system::firmware_revision();
    let uefi_revision = uefi::system::uefi_revision();

    println!("┌────────────────────────────────────────────┐");
    println!("│            TITOS Bootloader Info           │");
    println!("└────────────────────────────────────────────┘");
    println!(" Firmware Vendor   : {}", firmware_vendor);
    println!(" Firmware Revision : {:#010x}", firmware_revision);
    println!(" UEFI Spec Version : {}.{}", uefi_revision.major(), uefi_revision.minor());

    let mut kernel_file = {
        let mut fs = {
            let mut file_system = uefi::boot::get_image_file_system(uefi::boot::image_handle()).unwrap();

            file_system.open_volume().unwrap()
        };

        fs.open(cstr16!("kernel.elf"), FileMode::Read, FileAttribute::READ_ONLY).unwrap()
    };

    let kernel_size: usize = kernel_file.get_info::<FileInfo>(&mut [0;200]).unwrap().file_size() as usize;

    boot::allocate_pages(
        boot::AllocateType::Address(KERNEL_BASE_ADDR as u64),
        MemoryType::LOADER_DATA,
        (kernel_size + EFI_PAGE_SIZE - 1) / EFI_PAGE_SIZE
    ).unwrap();

    kernel_file.into_regular_file().unwrap().read(unsafe {
        core::slice::from_raw_parts_mut(KERNEL_BASE_ADDR as *mut u8, kernel_size)
    }).unwrap();

    let kernel_entry: extern "C" fn() = unsafe {
        core::mem::transmute(KERNEL_BASE_ADDR + 24)
    };

    unsafe {
        let _ = boot::exit_boot_services(None);
    }

    kernel_entry();

    loop {
    }
}