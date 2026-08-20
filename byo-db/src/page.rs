use crate::constants::PAGE_SIZE;
use crate::page_header::PageHeader;

pub struct Page<'a> {
    data: &'a mut [u8; PAGE_SIZE],
}

impl<'a> Page<'a> {
    pub fn new(data: &'a mut [u8; PAGE_SIZE]) -> Self {
        Self { data }
    }

    pub fn get_header(&self) -> PageHeader {
        PageHeader::from_bytes(self.data)
    }
}
