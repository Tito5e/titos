let BOOTLOADER_EFI = "./target/x86_64-unknown-uefi/release/titos.efi"
let MOUNT = "./mnt"

cp -f $BOOTLOADER_EFI $"($MOUNT)/efi/boot/bootx64.efi"

qemu-system-x86_64.exe -bios ./libs/OVMF.fd -hda fat:rw:./mnt