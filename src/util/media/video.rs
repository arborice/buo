use crate::prelude::*;
use matroska::{Info, Matroska};
use std::{fs::File, path::Path};

pub fn get_mkv_metadata(path: &Path) -> Result<MediaMeta> {
    let source = Matroska::open(File::open(path)?)?;
    let meta = source.info;
    let pretty_meta: MediaMeta = meta.try_into()?;
    Ok(pretty_meta)
}

impl TryInto<MediaMeta> for Info {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<MediaMeta, Self::Error> {
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
            title,
            duration,
            date: date_utc,
            ..Default::default()
        })
    }
}
