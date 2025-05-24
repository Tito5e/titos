#![no_std]
#![no_main]

use uefi::prelude::*;
use log::info;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();

    info!("Hello World");

    boot::stall(10_000_000);

    Status::SUCCESS
}