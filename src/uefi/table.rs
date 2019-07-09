#[derive(Clone, Copy)]
#[repr(C)]
pub struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    _reserved: u32,
}

impl TableHeader {
    pub fn signature(&self) -> u64 {
        self.signature
    }

    pub fn revision(&self) -> u32 {
        self.revision
    }

    pub fn header_size(&self) -> u32 {
        self.header_size
    }

    pub fn crc32(&self) -> u32 {
        self.crc32
    }
}

pub trait Table {
    fn header(&self) -> TableHeader;
}
