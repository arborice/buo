use crate::prelude::*;
use std::{collections::HashMap, path::Path};

// const MAX_CACHE_SIZE: usize = 600;
/// TODO: change this to og size
const MAX_CACHE_SIZE: usize = 128;

#[derive(Default, Deserialize, Serialize)]
pub struct CacheLookupKey {
    cache_index: usize,
}

#[derive(Deserialize, Serialize)]
pub struct HotCache {
    cache_lookup: HashMap<String, usize>,
    last_inserted_index: usize,
    /// TODO: #\[serde_as(serde_with="[None; MAX_CACHE_SIZE]")]
    entries: [Option<MediaMeta>; MAX_CACHE_SIZE],
}

impl Default for HotCache {
    fn default() -> Self {
        Self {
            cache_lookup: HashMap::new(),
            last_inserted_index: 0,
            entries: [None; MAX_CACHE_SIZE],
        }
    }
}

#[derive(Debug)]
enum CacheWrapResult {
    CacheFull,
    Linear(usize),
    Wrapped(usize),
}

use std::{error::Error, fmt};
impl fmt::Display for CacheWrapResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheWrapResult::CacheFull => {
                write!(f, "Cache is full. Only {} slots available.", MAX_CACHE_SIZE)
            }
            _ => unreachable!(),
        }
    }
}

impl Error for CacheWrapResult {}

impl HotCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, query: &str) -> Option<&MediaMeta> {
        let lookup_index = self.cache_lookup.get(query)?;
        self.entries.get(*lookup_index).map(|e| e.as_ref())?
    }

    pub fn inc_or_wrap_index(&self) -> CacheWrapResult {
        use CacheWrapResult::*;
        let mut i = self.last_inserted_index.checked_add(1).unwrap_or(0);

        while let Some(_) = self.entries.iter() {
            i += 1;
        }
        if i < MAX_CACHE_SIZE {
            return Wrapped(i + 1);
        }
        CacheFull
    }

    pub fn insert(&mut self, key: &str, entry: MediaMeta) -> Result<()> {
        if !self.cache_lookup.contains_key(key) {
            self.last_inserted_index = match self.inc_or_wrap_index() {
                CacheWrapResult::Linear(i) | CacheWrapResult::Wrapped(i) => i,
                _ => bail!(""),
            };

            self.entries
                .get_mut(self.last_inserted_index)
                .map(|e| e.replace(entry));
            Ok(())
        } else {
            bail!("failed insertion")
        }
    }

    pub fn retain_by_key(&mut self, mut functor: impl FnMut(&str) -> bool) {
        self.cache_lookup.retain(|key, _| functor(key));
    }

    pub fn retain_by_meta(&mut self, functor: fn(&MediaMeta) -> bool) {}

    pub fn rebase(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn extend(&mut self, extension: impl Iterator<Item = (String, MediaMeta)>) {
        match extension.size_hint() {
            (_, Some(extension_len)) | (extension_len, None) if extension_len > 0 => {
                self.resize_if_needed(extension_len)
            }
            _ => {}
        }

        for (key, meta) in extension {
            if let Some(index) = self.cache_lookup.get(&key) {
                self.entries.get_mut(*index).map(|e| e.replace(meta));
            } else {
                self.last_inserted_index += 1;
                self.entries
                    .get_mut(self.last_inserted_index)
                    .map(|e| e.replace(meta));
            }
        }
    }

    pub fn resize_if_needed(&mut self, added_len: usize) {
        if self.last_inserted_index + added_len >= self.entries.len() {
            self.entries
                .extend_from_slice(vec![None; added_len].as_mut_slice());
        }
    }

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

use std::fs::{read, write};
pub fn commit_cache_to_path(path: &Path, cache: HotCache) -> Result<()> {
    let serialized: Vec<u8> = bincode::serialize(&cache)?;
    write(path, serialized)?;
    Ok(())
}

pub fn retrieve_cache(path: &Path) -> Result<HotCache> {
    let byte_contents = read(path)?;
    let deserialized: HotCache = bincode::deserialize(&byte_contents)?;
    Ok(deserialized)
}
