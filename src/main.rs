#![no_std]
#![no_main]

mod uefi;

use core::panic::PanicInfo;

use uefi::{Status, Handle};
use uefi::system_table::*;
use uefi::runtime_services::*;

#[no_mangle]
pub extern "C" fn efi_main(_image: Handle, system_table: SystemTable) -> Status {
    let new_line = [0x000A, 0x0000].as_ptr();

    system_table.con_out().output_string(new_line);
    system_table.con_out().output_string(system_table.firmware_vendor());
    system_table.con_out().output_string(new_line);

    system_table.runtime_services().reset_system(ResetType::Shutdown, Status::SUCCESS);
    Status::SUCCESS
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
