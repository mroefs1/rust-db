/*
Pager — the bottom layer. It only knows about raw pages and the file.
Its vocabulary is "page number in, [u8; 4096] out" (or the reverse for writes).
It has no idea what a "cache" is, no idea what's hot or cold, no idea what a BufferPool even is.
It just does read_page(id) / write_page(id, bytes) / allocate_page() / free_page(id), against the actual file on disk,
with fsync per your Module 2 decision.
*/

use crate::{errors::page_error::PageError, page::Page};

mod freelist;

pub struct Pager {
    //placeholder to get the error in bufferpool constructor to fuck off
    id: u8,
}

impl Pager {
    pub fn new(id: u8) -> Self {
        Self { id: id }
    }
    pub fn read_page(id: u64) -> Result<Page, PageError> {
        todo!();
    }
}
