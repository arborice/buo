use crate::prelude::*;
use std::{collections::HashMap, path::Path};

#[derive(Deserialize, Serialize)]
pub(crate) struct HotCache {
    cache_lookup: HashMap<String, usize>,
    last_inserted_index: usize,
    entries: Vec<Option<MediaMeta>>,
}

impl HotCache {
    pub fn get(&self, query: &str) -> Option<&MediaMeta> {
        let lookup_index = self.cache_lookup.get(query)?;
        self.entries.iter().nth(*lookup_index).map(|e| e.as_ref())?
    }

    pub fn insert(&mut self, key: &str, entry: MediaMeta) -> Result<()> {
        if !self.cache_lookup.contains_key(key) {
            self.last_inserted_index += 1;
            self.entries[self.last_inserted_index].replace(entry);
            Ok(())
        } else {
            bail!("failed insertion")
        }
    }

    fn resize(&mut self) {
        if self.last_inserted_index == self.entries.len() {
            self.entries.reserve_exact(200);
            for _ in 0..200 {
                self.entries.push(None);
            }
        }
    }

    pub fn batch_query<P: AsRef<str>>(&self, queries: &Vec<P>) -> Option<Vec<&MediaMeta>> {
        let res_indexes: Vec<usize> = queries
            .iter()
            .filter_map(|q| match self.cache_lookup.get(q.as_ref()) {
                Some(ix) => Some(*ix),
                None => None,
            })
            .collect();

        if res_indexes.is_empty() {
            return None;
        }

        let batch_results = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(ix, meta)| {
                if res_indexes.contains(&ix) {
                    meta.as_ref()
                } else {
                    None
                }
            })
            .collect();
        Some(batch_results)
    }
}

use std::fs::{read, write};
pub(crate) fn commit_cache_to_path<Meta: Serialize>(path: &Path, cache: HotCache) -> Result<()> {
    let serialized: Vec<u8> = bincode::serialize(&cache)?;
    write(path, serialized)?;
    Ok(())
}

pub(crate) fn retrieve_cache(path: &Path) -> Result<HotCache> {
    let byte_contents = read(path)?;
    let deserialized: HotCache = bincode::deserialize(&byte_contents)?;
    Ok(deserialized)
}
