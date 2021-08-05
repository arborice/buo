use crate::prelude::*;
use std::{collections::HashMap, path::Path};

pub const MAX_CACHE_SIZE: usize = 1200;

#[serde_with::serde_as]
#[derive(Deserialize, Serialize)]
pub struct PersistentCache {
    cache_lookup: HashMap<String, usize>,
    last_inserted_index: usize,
    #[serde_as(as = "[Option<_>; MAX_CACHE_SIZE]")]
    entries: [Option<MediaMeta>; MAX_CACHE_SIZE],
}

#[must_use]
fn gen_empty_cache_entries() -> [Option<MediaMeta>; MAX_CACHE_SIZE] {
    use std::convert::TryInto;

    let mut cache_on_heap = Vec::with_capacity(MAX_CACHE_SIZE);
    for _ in 0..MAX_CACHE_SIZE {
        cache_on_heap.push(None);
    }

    cache_on_heap
        .try_into()
        .unwrap_or_else(|_| panic!("Cache size must be {}", MAX_CACHE_SIZE))
}

impl PersistentCache {
    pub fn new() -> Self {
        Self {
            cache_lookup: HashMap::new(),
            last_inserted_index: 0,
            entries: gen_empty_cache_entries(),
        }
    }

    pub fn get(&self, query: &str) -> Option<&MediaMeta> {
        let lookup_index = self.cache_lookup.get(query)?;
        self.entries.get(*lookup_index).map(|e| e.as_ref())?
    }

    fn next_insertion_index(&mut self) -> Result<()> {
        self.last_inserted_index = self
            .entries
            .iter()
            .position(|i| i.is_none())
            .ok_or_else(|| anyhow!("Cache of size {} is full", MAX_CACHE_SIZE))?;
        Ok(())
    }

    pub fn insert(&mut self, key: &str, entry: MediaMeta) -> Result<()> {
        if !self.cache_lookup.contains_key(key) {
            self.entries
                .get_mut(self.last_inserted_index)
                .map(|e| e.replace(entry));
            Ok(())
        } else {
            bail!("{} is an existing key", key)
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<MediaMeta> {
        let index = self.cache_lookup.get(key)?;
        if *index < MAX_CACHE_SIZE {
            self.entries[*index].take()
        } else {
            None
        }
    }

    pub fn retain(&mut self, functor: fn(&str, Option<&MediaMeta>) -> bool) {
        let mut keys_to_remove = vec![];

        for (key, index) in self.cache_lookup.iter_mut() {
            let maybe_meta = self.entries.get(*index).map(|opt| opt.as_ref()).flatten();
            let must_remove = functor(key, maybe_meta);

            if must_remove {
                if let Some(entry) = self.entries.get_mut(*index) {
                    *entry = None;
                }
                keys_to_remove.push(key.clone());
            }
        }

        for key in keys_to_remove {
            self.cache_lookup.remove(key.as_str());
        }
    }

    #[must_use]
    pub fn batch_query<P: AsRef<str>>(&self, queries: &[P]) -> Option<Vec<&MediaMeta>> {
        let res_indexes: Vec<usize> = queries
            .iter()
            .filter_map(|q| self.cache_lookup.get(q.as_ref()).copied())
            .collect();

        if res_indexes.is_empty() {
            return None;
        }

        let batch_results = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(ix, meta)| res_indexes.contains(&ix).opt_and(meta.as_ref()))
            .collect();
        Some(batch_results)
    }
}

impl Default for PersistentCache {
    fn default() -> Self {
        Self::new()
    }
}

use std::fs::{read, write};
pub fn commit_cache_to_path(path: &Path, cache: PersistentCache) -> Result<()> {
    let serialized: Vec<u8> = bincode::serialize(&cache)?;
    write(path, serialized)?;
    Ok(())
}

use bincode::ErrorKind as BinErr;
use std::io::ErrorKind as IoErr;

fn init_cache(path: &Path) -> Result<PersistentCache> {
    let dfl_cache = PersistentCache::new();
    let serialized_cache: Vec<u8> = bincode::serialize(&dfl_cache)?;
    write(path, serialized_cache)?;
    Ok(dfl_cache)
}

fn is_init_candidate(err: &BinErr) -> bool {
    use IoErr::{AlreadyExists, InvalidData, InvalidInput, NotFound, PermissionDenied};
    match err {
        BinErr::Io(e)
            if matches!(
                e.kind(),
                NotFound | PermissionDenied | AlreadyExists | InvalidInput | InvalidData
            ) =>
        {
            true
        }
        _ => false,
    }
}

pub fn retrieve_or_init_cache(path: &Path) -> Result<PersistentCache> {
    let byte_contents = read(path)?;
    let deserialized: Result<PersistentCache, _> = bincode::deserialize(&byte_contents);

    match deserialized {
        Err(e) if is_init_candidate(&*e) => init_cache(path),
        Err(e) => bail!(e),
        Ok(res) => Ok(res),
    }
}
