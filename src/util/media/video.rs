use crate::prelude::*;
use matroska::{Info, Matroska};
use std::fs::File;

use crate::util::{iso4::iso4_meta, traits::ExtCallback};
pub struct VideoAnalyzer;

impl ExtCallback for VideoAnalyzer {
    fn try_get_meta(&self, path: &std::path::Path) -> Result<Option<MediaMeta>> {
        let ext = get_file_ext(path).ok_or_else(|| anyhow!("File has no file extension!"))?;
        let file_ext = FileExt::from(ext);

        if file_ext.is_matroska() {
            let source = Matroska::open(File::open(path)?)?;
            let meta = source.info;

            let file_name = get_file_name(path);
            let pretty_meta = meta.into_meta(file_name);
            return Ok(pretty_meta);
        }

        if file_ext.is_iso4() {
            return iso4_meta(path);
        }

        bail!("Unsupported file type")
    }
}

impl IntoMeta for Info {
    fn into_meta(self, file_name: String) -> Option<MediaMeta> {
        let Info {
            title,
            duration,
            date_utc,
            ..
        } = self;

        // return early if no fields present
        let _no_avail_meta = title
            .as_ref()
            .and(duration.as_ref())
            .and(date_utc.as_ref())?;

        Some(MediaMeta {
            file_name,
            title,
            duration,
            date: date_utc.map(DateKind::Chrono),
            ..Default::default()
        })
    }
}
