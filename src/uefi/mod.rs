pub mod image;
pub mod proto;
pub mod system_table;
pub mod table_header;

use core::ffi::c_void;

#[repr(usize)]
pub enum Status {
    SUCCESS = 0,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Handle(*const c_void);
