use crate::uefi::Status;

#[repr(C)]
pub struct SimpleTextOutput {
    reset: extern "win64" fn(this: &SimpleTextOutput, extended: bool) -> Status,
    output_string: extern "win64" fn(this: &SimpleTextOutput, string: *const u16) -> Status,
}

impl SimpleTextOutput {
    pub fn reset(&mut self, extended: bool) -> Status {
        (self.reset)(self, extended)
    }

    pub fn output_string(&mut self, string: *const u16) -> Status {
        (self.output_string)(self, string)
    }
}
