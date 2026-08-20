use std::collections::{HashMap, VecDequeue};

pub const PAGE_SIZE: usize = 4096;

pub struct FrameMetadata {
    pub data: [u8; PAGE_SIZE],
    pub pin_count: u32,
    pub referenced: bool,
}

pub struct BufferPool {
    //registry hashmap: map page id to raw data/eviction metadata
    pub frames: HashMap<u64, FrameMetadata>,

    //History queue
    pub history_queue: VecDequeue<u32>,

    //Cache Queue
    pub cache_queue: VecDequeue<u32>,

    pub pool_capacity: usize,
    pub max_history_capacity: usize,
}
