/*

Engine — the top layer, currently a thin shell. It owns the BufferPool (which transitively owns the Pager)
and exposes whatever the rest of the project — your future B-tree, later the query layer — actually needs as a stable entry point.
Nothing above Engine should ever need to know a Pager exists at all.

*/

use crate::buffer_pool::BufferPool;

pub struct Engine {
    pub buffer_pool: BufferPool,
}

impl Engine {}
