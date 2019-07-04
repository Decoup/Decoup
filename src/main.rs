#![no_std]
#![no_main]

mod uefi;

use core::panic::PanicInfo;

use uefi::{Status, Handle};
use uefi::system_table::*;

#[no_mangle]
pub extern "C" fn efi_main(_image: Handle, system_table: SystemTable) -> Status {
    let mut buf = [0u16; 32];

    for (i, c) in "Hello World!\n".encode_utf16().enumerate() {
        buf[i] = c;
    }

    system_table.con_out().reset(false);
    system_table.con_out().output_string(&buf);

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
