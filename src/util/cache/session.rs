use crate::prelude::*;
use tinyvec::{array_vec, ArrayVec};

const MIN_CACHE_SIZE: usize = 64;

#[derive(Deserialize, Serialize)]
pub struct LiveCache {
    entries: ArrayVec<[MediaMeta; MIN_CACHE_SIZE]>,
    last_inserted_index: usize,
}

impl Default for LiveCache {
    fn default() -> Self {
        Self {
            entries: array_vec!([MediaMeta; MIN_CACHE_SIZE]),
            last_inserted_index: 0,
        }
    }
}

impl LiveCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn contains(&self, meta: &MediaMeta) -> bool {
        self.entries.iter().any(|e| e == meta)
    }

    pub fn has_capacity(&self) -> bool {
        self.entries.len() < MIN_CACHE_SIZE
    }

    fn increment(&mut self) {
        if self.entries.len() < MIN_CACHE_SIZE - 1 {
            self.last_inserted_index += 1;
        } else {
            self.last_inserted_index = 0;
        }
    }

    pub fn find<P>(&self, fun: P) -> Option<&MediaMeta>
    where
        P: FnMut(&&MediaMeta) -> bool,
    {
        self.entries.iter().find(fun)
    }

    pub fn entries(&self) -> impl Iterator<Item = &MediaMeta> {
        self.entries.iter()
    }

    pub fn insert(&mut self, meta: MediaMeta) {
        if self.has_capacity() {
            self.entries.push(meta);
        } else {
            self.entries.remove(self.last_inserted_index);
            self.entries.insert(self.last_inserted_index, meta);

            self.increment()
        }
    }
}
