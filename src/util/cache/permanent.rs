use crate::prelude::*;
use std::{collections::HashMap, path::Path};

#[derive(Default, Deserialize, Serialize)]
pub struct HotCache {
    cache_lookup: HashMap<String, usize>,
    last_inserted_index: usize,
    entries: Vec<Option<MediaMeta>>,
}

impl HotCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, query: &str) -> Option<&MediaMeta> {
        let lookup_index = self.cache_lookup.get(query)?;
        self.entries.get(*lookup_index).map(|e| e.as_ref())?
    }

    pub fn insert(&mut self, key: &str, entry: MediaMeta) -> Result<()> {
        if !self.cache_lookup.contains_key(key) {
            self.last_inserted_index += 1;
            self.entries
                .get_mut(self.last_inserted_index)
                .map(|e| e.replace(entry));
            Ok(())
        } else {
            bail!("failed insertion")
        }
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
