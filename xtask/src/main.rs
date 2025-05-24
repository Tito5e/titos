use std::{path::PathBuf, process::Command};
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if args[1] == "build" {
            build()?;
        } else if args[1] == "run" {
            run()?;
        } else {
            println!("Usage: cargo run (build|run)");
        }
    }

    Ok(())
}

fn build() -> Result<()> {
    println!("====================");
    println!("STARTING BUILD TITOS");
    println!("====================");

    let status = Command::new("cargo")
        .args(["build", "--release", "-p", "boot", "--target", "x86_64-unknown-uefi"])
        .status()?;

    assert!(status.success(), "{}", status);

    let status = Command::new("cargo")
        .args(["build", "--release", "-p", "kernel", "--target", "x86_64-unknown-none"])
        .status()?;

    assert!(status.success(), "{}", status);

    let bootable_folder = PathBuf::from("mnt");

    std::fs::remove_dir_all(&bootable_folder)?;
    std::fs::create_dir_all(&bootable_folder.join("EFI/BOOT"))?;

    std::fs::copy(
        "target/x86_64-unknown-uefi/release/boot.efi",
        bootable_folder.join("EFI/BOOT/BOOTX64.EFI"),
    )?;

    std::fs::copy(
        "target/x86_64-unknown-none/release/kernel",
        bootable_folder.join("kernel.elf"),
    )?;

    Ok(())
}

fn run() -> Result<()> {
    Command::new("qemu-system-x86_64")
        .args(["-bios", "./libs/OVMF.fd", "-drive", "format=raw,file=fat:rw:./mnt"])
        .status()?;

    Ok(())
}