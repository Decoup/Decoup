#![no_std]
#![no_main]

mod uefi;

use core::ffi::c_void;
use core::panic::PanicInfo;

use uefi::proto::simple_text_output::*;
use uefi::Status;

#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    _reserved: u32,
}


#[repr(C)]
pub struct EfiSystemTable {
    pub header: EfiTableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    _con_in: usize,
    pub console_out_handle: EfiHandle,
    pub con_out: *mut SimpleTextOutput,
    pub standard_error_handle: EfiHandle,
    _std_err: usize,// TBD
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EfiHandle(*mut c_void);

#[no_mangle]
pub extern "C" fn efi_main(_image: EfiHandle, st: EfiSystemTable) -> Status {
    let stdout: &mut SimpleTextOutput = unsafe { &mut *(st.con_out) };

    let mut buf = [0u16; 32];

    for (i, c) in "Hello World!\n".encode_utf16().enumerate() {
        buf[i] = c;
    }

    stdout.reset(false);
    stdout.output_string(&buf);

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
