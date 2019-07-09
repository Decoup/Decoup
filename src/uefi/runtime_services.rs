use crate::uefi::Handle;
use crate::uefi::Status;
use crate::uefi::table::*;

use core::ffi::c_void;

#[repr(C)]
pub struct RuntimeServices {
    header: TableHeader,

    _get_time: usize,
    _set_time: usize,
    _get_wakeup_time: usize,
    _set_wakeup_time: usize,

    _set_virtual_address_map: usize,
    _convert_pointer: usize,

    _get_variable: usize,
    _get_next_variable_name: usize,
    _set_variable: usize,

    _get_next_high_monotonic_count: usize,
    reset_system: extern "win64" fn(reset_type: ResetType, status: Status, data_size: usize, data_ptr: usize) -> !,

    _update_capsule: usize,
    _query_capsule_capabilities: usize,

    _query_variable_info: usize
}

impl Table for RuntimeServices {
    fn header(&self) -> TableHeader {
        self.header
    }
}

impl RuntimeServices {
    pub fn reset_system(&mut self, reset_type: ResetType, status: Status) -> ! {
        (self.reset_system)(reset_type, status, 0, 0)
    }
}

#[repr(usize)]
pub enum ResetType {
    Cold,
    Warm,
    Shutdown,
    PlatformSpecific
}
