pub(crate) mod prelude;
pub(crate) mod util;
pub use util::{dirs::DirMeta, media::meta::MediaMeta, ExportKind, ExportedJson};

use std::path::Path;
use util::{dirs::get_dir_meta, media::dispatch_meta_fn};

pub fn buo_media_query(query: &Path) -> anyhow::Result<MediaMeta> {
    assert!(query.is_file());
    dispatch_meta_fn(query)
        .ok_or_else(|| anyhow::anyhow!("Not a supported file type"))
        .and_then(|dispatcher| dispatcher.try_get_meta(query))
}

pub fn buo_dir_meta(query: &Path) -> anyhow::Result<DirMeta> {
    assert!(query.is_dir());
    get_dir_meta(query)
}
