use crate::prelude::*;

pub trait ExtCallback: Send + Sync {
    fn try_get_meta(&self, path: &std::path::Path) -> Result<Option<MediaMeta>>;
}

pub trait IntoMeta {
    fn into_meta(self, file_name: String) -> Option<MediaMeta>;
}
