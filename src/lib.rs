pub(crate) mod prelude;
pub(crate) mod util;
pub use util::{
    cache::{
        commit_cache_to_path, get_initial_entries, replace_invalid_entries, retrieve_or_init_cache,
        LiveCache, PersistentCache, MAX_CACHE_SIZE,
    },
    dev::LangStats,
    dirs::DirMeta,
    json_out::{ExportKind, ExportedJson},
    media::meta::MediaMeta,
};

use anyhow::{bail, Result};
use std::path::Path;
use util::{dirs::get_dir_meta, media::dispatch_meta_fn};

pub fn buo_media_query(query: &Path) -> Result<Option<MediaMeta>> {
    if !query.is_file() {
        bail!("{} is not a regular file!", query.display());
    }

    dispatch_meta_fn(query)
        .ok_or_else(|| anyhow::anyhow!("Not a supported file type"))
        .and_then(|dispatcher| dispatcher.try_get_meta(query))
}

pub fn buo_dir_meta(query: &Path) -> Result<DirMeta> {
    if !query.is_dir() {
        bail!("{} is not a directory!", query.display());
    }
    get_dir_meta(query)
}

pub fn force_init_cache(path: &Path) -> Result<()> {
    commit_cache_to_path(path, PersistentCache::new())
}
