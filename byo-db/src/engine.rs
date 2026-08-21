use crate::buffer_pool::BufferPool;
use crate::constants::PAGE_SIZE;
use crate::pager::Pager;

pub struct Engine {
    pub pager: Pager,
    pub buffer_pool: BufferPool,
}

impl Engine {}
