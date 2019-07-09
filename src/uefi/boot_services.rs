use crate::uefi::Handle;
use crate::uefi::table::*;

#[repr(C)]
pub struct BootServices {
    header: TableHeader,

    _raise_tpl: usize,
    _restore_tpl: usize,

    _allocate_pages: usize,
    _free_pages: usize,
    _get_memory_map: usize,
    _allocate_pool: usize,
    _free_pool: usize,

    _create_event: usize,
    _set_timer: usize,
    _wait_for_event: usize,
    _signal_event: usize,
    _close_event: usize,
    _check_event: usize,

    _install_protocol_interface: usize,
    _reinstall_protocol_interface: usize,
    _uninstall_protocol_interface: usize,
    _handle_protocol: usize,
    _reserved: usize,
    _register_protocol_notify: usize,
    _locate_handle: usize,
    _locate_device_path: usize,
    _install_configuration_table: usize,

    _load_image: usize,
    _start_image: usize,
    _exit: usize,
    _unload_image: usize,
    _exit_boot_services: usize,

    _get_next_monotonic_count: usize,
    _stall: usize,
    _set_watchdog_timer: usize,

    _connect_controller: usize,
    _disconnect_controller: usize,

    _open_protocol: usize,
    _close_protocol: usize,
    _open_protocol_information: usize,

    _protocols_per_service: usize,
    _locate_handle_buffer: usize,
    _locate_protocol: usize,
    _install_multiple_protocol_interfaces: usize,
    _uninstall_multiple_protocol_interfaces: usize,

    _calculate_crc32: usize,

    _copy_mem: usize,
    _set_mem: usize,
    _create_event_ex: usize
}

impl Table for BootServices {
    fn header(&self) -> TableHeader {
        self.header
    }
}
