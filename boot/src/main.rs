#![no_std]
#![no_main]

mod kernel;

use kernel::load_kernel;
use uefi::{prelude::*, println};

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

    let kernel_entry = load_kernel(KERNEL_BASE_ADDR, EFI_PAGE_SIZE);

    unsafe {
        let _ = boot::exit_boot_services(None);
    }

    kernel_entry();

    loop {
    }
}