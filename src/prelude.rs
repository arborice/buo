pub use crate::util::media::meta::{DateKind, IntoMeta, MediaMeta};
pub use anyhow::{anyhow, bail, Result};
pub use serde::{Deserialize, Serialize};

pub fn get_file_name(path: &std::path::Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default()
}
