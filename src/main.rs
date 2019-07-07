#![no_std]
#![no_main]

mod uefi;

use core::panic::PanicInfo;

use uefi::{Status, Handle};
use uefi::system_table::*;

#[no_mangle]
pub extern "C" fn efi_main(_image: Handle, system_table: SystemTable) -> Status {
    system_table.con_out().reset(false);
    system_table.con_out().output_string(system_table.firmware_vendor());

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
