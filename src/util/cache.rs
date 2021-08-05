pub mod local;
pub mod session;

pub use local::{commit_cache_to_path, retrieve_or_init_cache, PersistentCache, MAX_CACHE_SIZE};
pub use session::LiveCache;

use crate::prelude::*;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

/// Recursively accrues paths validated by a closure, and fails eagerly
fn acc_valid_paths(
    dir_path: &Path,
    acc: &mut Vec<PathBuf>,
    len_limit: &mut usize,
    validator: impl Fn(&Path) -> bool,
) -> Result<()> {
    for ent in read_dir(dir_path)? {
        let path = ent?.path();
        if path.is_dir() {
            acc_valid_paths(&path, acc, len_limit, &validator)?;
        }

        if validator(&path) {
            *len_limit -= 1;
            acc.push(path);
        }
        if *len_limit == 0 {
            return Ok(());
        }
    }
    Ok(())
}

pub fn replace_invalid_entries(
    root_path: &Path,
    paths: &mut [PathBuf],
    validator: impl Fn(&Path) -> bool,
) -> Result<()> {
    paths.sort_unstable_by(|p1, p2| validator(p2).cmp(&validator(p1)));

    if let Some(first_invalid_entry) = paths.iter().position(|p| !validator(p)) {
        let mut len_limit = paths.len() - first_invalid_entry;
        let mut new_valid_paths = Vec::with_capacity(len_limit);

        acc_valid_paths(root_path, &mut new_valid_paths, &mut len_limit, validator)?;
        paths[first_invalid_entry..].swap_with_slice(new_valid_paths.as_mut_slice());
    }
    Ok(())
}

pub fn get_initial_entries(
    root_path: &Path,
    mut len_limit: usize,
    validator: impl Fn(&Path) -> bool,
) -> Result<Vec<PathBuf>> {
    let mut entries = Vec::with_capacity(len_limit);
    acc_valid_paths(root_path, &mut entries, &mut len_limit, validator)?;
    Ok(entries)
}
