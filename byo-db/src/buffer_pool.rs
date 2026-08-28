/*
BufferPool — the middle layer, and the one doing the interesting work.
It's the only thing that's allowed to talk to Pager directly.
When something asks it for page N, it first checks frames (the hashmap) — if it's already resident in memory,
hand back that copy, no disk hit at all. If it's not resident,
that's the moment it finally calls down to Pager::read_page, then inserts the result into frames,
and threads the page ID through the history/cache queue machinery you designed.
When it needs to evict to make room, it runs the sweep logic (history-first, then cache with referenced-bit second chances),
and if the evicted frame is dirty, that's the moment it calls Pager::write_page to flush it out before dropping it from memory.
*/

use crate::page;
use crate::pager::Pager;
use crate::{constants::PAGE_SIZE, errors::page_error::PageError};
use std::collections::{HashMap, VecDeque};

pub struct FrameMetadata {
    pub data: Box<[u8; PAGE_SIZE]>,
    pub referenced: bool, //if true don't evict
    pub is_dirty: bool,   //if true needs to be written to disk
}

pub struct BufferPool {
    //pager lifetime
    pager: Pager,
    ///registry hashmap: map page id to raw data/eviction metadata
    pub frames: HashMap<u64, FrameMetadata>,
    //History queue: Pass in capacity in constructor
    pub history_queue: VecDeque<u64>,
    //Cache Queue: Pass in capacity in constructor
    pub cache_queue: VecDeque<u64>,
    pub pool_capacity: usize,
    pub max_history_capacity: usize,
}

impl BufferPool {
    ///Create the BufferPool. Should only be called once
    ///Constructs frames hashmap, history_queue, cache_queue
    ///Creates the pager that it will use via Pager::new()
    pub fn new(pool_cap: usize, history_cap: usize) -> Self {
        Self {
            pager: Pager::new(1), //id is placeholder to avoid errors
            frames: HashMap::new(),
            history_queue: VecDeque::with_capacity(history_cap),
            cache_queue: VecDeque::with_capacity(pool_cap - history_cap),
            pool_capacity: pool_cap,
            max_history_capacity: history_cap,
        }
    }
    /*
        BufferPool Methods the Engine can actually call (for now)
        retreive_page, new_page, flush_page, flush_all_pages
    */

    ///Fetches a page from the cache (or loads it from disk via the pager if it's a cache miss),
    ///If in history queue, promote to cache queue.
    ///If in cache queue, set referenced to true and place on back of queue,
    ///because this guarantees that left most ele is the least recently used.
    ///Returns page's data from the FrameMetadata container.
    pub fn retrieve_page(&mut self, page_id: u64) -> Result<Box<[u8; PAGE_SIZE]>, PageError> {
        //if cache hit
        if let Some(mut frame) = self.frames.remove(&page_id) {
            //if in history promote
            if self.history_queue.contains(&page_id) {
                self.promote(page_id);
            }
            //if in cache instead move to the back so it doesn't get evicted
            if self.cache_queue.contains(&page_id) {
                frame.referenced = true;
                self.cache_queue.remove(page_id as usize);
                self.cache_queue.push_back(page_id);
            }
            //put back in hashmap and return frame data
            self.frames.insert(page_id, frame);
            Ok((self.frames.get(&page_id).data).clone())
        }
        //if no cache hit -> go to Pager and get from disk
        //then insert into history queue
        let loaded_page = self.pager.read_page(&page_id);
        if self.history_queue.len() == self.history_queue.capacity() {
            self.eviction_sweep();
        }
        self.history_queue.push_back(page_id);
        Ok(loaded_page.data)
    }

    ///For promotion of a page from history queue to cache queue.
    ///If the cache queue is full, calls cache_eviction to make a space
    fn promote(&mut self, page_id: u64) {
        if self.cache_queue.len() == self.cache_queue.capacity() {
            self.cache_eviction();
        }
        self.history_queue.remove(page_id as usize);
        self.cache_queue.push_back(page_id);
    }

    ///Allocates a brand-new page on disk via the pager, loads it into a buffer frame,
    ///adds to the history queue, and returns both the new page id and frame to the engine
    pub fn new_page(&mut self, page_id: u64) -> Result<FrameMetadata, PageError> {
        todo!();
    }

    //call on the pager to flush a specific page id
    pub fn flush_page(&mut self, page_id: u64) {}

    //helper function to avoid taking &mut self
    //The reason why, apparently is that the borrow checker will be upset if we borrow all of self
    fn flush_frame(pager: &mut Pager, id: u64, frame: &FrameMetadata) {}

    //call on the pager to flush all the dirty pages
    pub fn flush_all_pages(&mut self) {}

    /*
    if history and cache queue are full:

    1) Try the history queue first. Look at the front of history_queue.
    If it's non-empty, pop that page ID — that page is evicted immediately (first-touch pages get no second chance;

    2) If History queue is empty fall back to cache queue:
        i) pop a frame from the front and check it's ref bit
            a) if ref == true, flip the bit and push to the back
                I) repeat i
            b) if ref == false, cold page -> evict. If evict page is dirty -> flush it
    */

    ///If history queue isn't empty, remove an id from it.
    ///Remove the associated FrameMetadata from frames, and write to disk if it's dirty.
    /// If history queue is empty, call cache_eviction    
    fn eviction_sweep(&mut self) {
        if !self.history_queue.is_empty() {
            if let Some(history_id) = self.history_queue.pop_front() {
                if let Some(candidate_frame) = self.frames.remove(&history_id) {
                    if candidate_frame.is_dirty {
                        self.flush_page(history_id);
                    }
                }
            }
            return;
        }
        self.cache_eviction();
    }

    ///If cache is full and needs an eviction for a promoted member, call this directly
    ///For inserting a new page use eviction_sweep to process history queue evictions.
    fn cache_eviction(&mut self) {
        let mut counter = 0;
        while let Some(id) = self.cache_queue.pop_front() {
            //been through whole cache queue pop from front
            if counter == self.pool_capacity {
                self.frames.remove(&id);
                self.cache_queue.pop_front();
            }
            if let Some(mut candidate_frame) = self.frames.remove(&id) {
                if candidate_frame.referenced {
                    candidate_frame.referenced = false;
                    self.cache_queue.push_back(id);
                    self.frames.insert(id, candidate_frame);
                    counter += 1;
                    continue;
                }
                if candidate_frame.is_dirty {
                    self.flush_page(id);
                }
                self.frames.remove(&id);
                return;
            }
        }
    }
}
