use crate::constants::PAGE_SIZE;
use crate::pager::Pager;
use std::collections::{HashMap, VecDeque};

pub struct FrameMetadata {
    pub data: Box<[u8; PAGE_SIZE]>,
    pub pin_count: u32, //pinned or not? important for future multithreading
    pub referenced: bool,
    pub is_dirty: bool, //does this need to be written to disk?
}

pub struct BufferPool<'a> {
    //pager lifetime
    pager: &'a mut Pager,

    //registry hashmap: map page id to raw data/eviction metadata
    pub frames: HashMap<u64, FrameMetadata>,

    //History queue
    pub history_queue: VecDeque<u32>,

    //Cache Queue
    pub cache_queue: VecDeque<u32>,

    pub pool_capacity: usize,
    pub max_history_capacity: usize,
}
