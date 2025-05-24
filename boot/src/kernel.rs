use uefi::boot::MemoryType;
use uefi::{boot, cstr16};
use uefi::proto::media::file::{File as _, FileAttribute, FileInfo, FileMode};

pub fn load_kernel(kernel_start: usize, efi_page_size: usize) -> extern "C" fn() {
    let mut kernel_file = {
        let mut fs = {
            let mut file_system = uefi::boot::get_image_file_system(uefi::boot::image_handle()).unwrap();

            file_system.open_volume().unwrap()
        };

        fs.open(cstr16!("kernel.elf"), FileMode::Read, FileAttribute::READ_ONLY).unwrap()
    };

    let kernel_size: usize = kernel_file.get_info::<FileInfo>(&mut [0;200]).unwrap().file_size() as usize;

    boot::allocate_pages(
        boot::AllocateType::Address(kernel_start as u64),
        MemoryType::LOADER_DATA,
        (kernel_size + efi_page_size - 1) / efi_page_size
    ).unwrap();

    kernel_file.into_regular_file().unwrap().read(unsafe {
        core::slice::from_raw_parts_mut(kernel_start as *mut u8, kernel_size)
    }).unwrap();

    let kernel_entry: extern "C" fn() = unsafe {
        core::mem::transmute(kernel_start + 24)
    };

    kernel_entry
}