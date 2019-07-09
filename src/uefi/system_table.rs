use crate::uefi::Handle;
use crate::uefi::table::*;
use crate::uefi::proto::simple_text_output::*;
use crate::uefi::boot_services::*;
use crate::uefi::runtime_services::*;

#[repr(C)]
pub struct SystemTable {
    header: TableHeader,
    firmware_vendor: *const u16,
    firmware_revision: u32,
    _console_in_handle: Handle,
    _con_in: usize,
    _console_out_handle: Handle,
    con_out: *mut SimpleTextOutput,
    _standard_error_handle: Handle,
    std_err: *mut SimpleTextOutput,
    runtime_services: *mut RuntimeServices,
    boot_services: *mut BootServices,
    _number_of_table_entries: usize, // TODO
    _efi_configuration_table: usize // TODO
}

impl SystemTable {
    pub fn firmware_vendor(&self) -> *const u16 {
        self.firmware_vendor
    }

    pub fn firmware_revision(&self) -> u32 {
        self.firmware_revision
    }

    pub fn con_out(&self) -> &mut SimpleTextOutput {
        unsafe { &mut *(self.con_out) }
    }

    pub fn std_err(&self) -> &mut SimpleTextOutput {
        unsafe { &mut *(self.std_err) }
    }

    pub fn runtime_services(&self) -> &mut RuntimeServices {
        unsafe { &mut *(self.runtime_services) }
    }

    pub fn boot_services(&self) -> &mut BootServices {
        unsafe { &mut *(self.boot_services) }
    }
}

impl Table for SystemTable {
    fn header(&self) -> TableHeader {
        self.header
    }
}
