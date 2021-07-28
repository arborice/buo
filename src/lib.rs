pub(crate) mod prelude;
pub(crate) mod util;
pub use util::{
    cache::{
        commit_cache_to_path, get_initial_entries, replace_invalid_entries, retrieve_cache,
        PersistentCache,
    },
    dev::LangStats,
    dirs::DirMeta,
    json_out::{ExportKind, ExportedJson},
    media::meta::MediaMeta,
};

use std::path::Path;
use util::{dirs::get_dir_meta, media::dispatch_meta_fn};

pub fn buo_media_query(query: &Path) -> anyhow::Result<Option<MediaMeta>> {
    assert!(query.is_file());
    dispatch_meta_fn(query)
        .ok_or_else(|| anyhow::anyhow!("Not a supported file type"))
        .and_then(|dispatcher| dispatcher.try_get_meta(query))
}

pub fn buo_dir_meta(query: &Path) -> anyhow::Result<DirMeta> {
    if !query.is_dir() {
        panic!("{} is not a directory!", query.display());
    }
    get_dir_meta(query)
}

pub fn retrieve_cache_or_try_init(path: &Path) -> anyhow::Result<Option<PersistentCache>> {
    let try_fetch_existing = retrieve_cache(path);
    if path.exists() {
        let existing = try_fetch_existing?;
        Ok(Some(existing))
    } else {
        Ok(None)
    }
}

pub fn force_init_cache(path: &Path) -> anyhow::Result<()> {
    commit_cache_to_path(path, PersistentCache::new())
}
