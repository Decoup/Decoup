use crate::uefi::Handle;
use crate::uefi::table_header::*;
use crate::uefi::proto::simple_text_output::*;

#[repr(C)]
pub struct SystemTable {
    _header: TableHeader,
    _firmware_vendor: *const u16,
    _firmware_revision: u32,
    _console_in_handle: Handle,
    _con_in: usize,
    _console_out_handle: Handle,
    con_out: *mut SimpleTextOutput,
    _standard_error_handle: Handle,
    std_err: *mut SimpleTextOutput,
    _runtime_services: usize, // TODO
    _boot_services: usize, // TODO
    _number_of_table_entries: usize, // TODO
    _efi_vonfiguration_table: usize // TODO
}

impl SystemTable {
    pub fn con_out(&self) -> &mut SimpleTextOutput {
        unsafe { &mut *(self.con_out) }
    }
}
