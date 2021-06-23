use crate::prelude::*;
use matroska::{Info, Matroska};
use std::{fs::File, path::Path};

pub fn get_mkv_metadata(path: &Path) -> Result<MediaMeta> {
    let source = Matroska::open(File::open(path)?)?;
    let meta = source.info;

    let file_name = get_file_name(path);
    let pretty_meta: MediaMeta = meta.into_meta(file_name)?;
    Ok(pretty_meta)
}

impl IntoMeta for Info {
    fn into_meta(self, file_name: String) -> Result<MediaMeta> {
        let Info {
            title,
            duration,
            date_utc,
            ..
        } = self;

        if title
            .as_ref()
            .and(duration.as_ref())
            .and(date_utc.as_ref())
            .is_none()
        {
            return Err(anyhow!("No metadata available!"));
        }

        Ok(MediaMeta {
            file_name,
            title,
            duration,
            date: date_utc.map(DateKind::Chrono),
            ..Default::default()
        })
    }
}
