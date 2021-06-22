use crate::prelude::*;
use std::{fs::File, path::Path};

use symphonia::core::{
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::{MetadataOptions, StandardTagKey, Tag},
    probe::{Hint, ProbeResult},
};

pub fn get_audio_metadata(path: &Path) -> Result<Option<MediaMeta>> {
    let mut hint = Hint::new();

    if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
        hint.with_extension(ext);
    }

    let source = Box::new(File::open(path)?);
    let media_source_stream = MediaSourceStream::new(source, Default::default());

    let format_opts = FormatOptions::default();
    let meta_opts = MetadataOptions::default();

    let probe = symphonia::default::get_probe().format(
        &hint,
        media_source_stream,
        &format_opts,
        &meta_opts,
    )?;

    let pretty_printed_meta = pretty_meta(probe);
    Ok(pretty_printed_meta)
}

fn pretty_meta(probe: ProbeResult) -> Option<String> {
    let meta = probe
        .format
        .metadata()
        .current()
        .or_else(|| probe.metadata.current())?;

    let tags = meta.tags();
    pretty_tags(tags)
}

impl TryInto<MediaMeta> for &[Tag] {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MediaMeta, Self::Error> {
        if self.is_empty() {
            return Err(anyhow!("No metadata available!"));
        }

        let (mut known, mut unknown): (Vec<_>, Vec<_>) =
            self.iter().partition(|tag| tag.is_known());

        let mut title = None;
        let mut author = None;
        let mut duration = None;
        let mut date = None;
        for (key, value) in known
            .drain(..)
            .map(|Tag { std_key, value, .. }| (std_key.unwrap(), value))
        {
            match key {
                StandardTagKey::Artist => {
                    author.replace(value.to_string());
                }
                StandardTagKey::AlbumArtist | StandardTagKey::Composer | StandardTagKey::Writer => {
                    if author.is_none() {
                        author.replace(value.to_string());
                    }
                }
                StandardTagKey::Duration => {
                    duration.replace(value.to_string());
                }
                StandardTagKey::Date => {
                    date.replace(value);
                }
                _ => {}
            }
        }

        let extra = unknown
            .drain(..)
            .map(|Tag { key, value, .. }| format!("{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("\n");
        let mut prettified_tags = vec![];

        Ok(MediaMeta {
            title,
            author,
            duration,
            date,
            extra,
        })
    }
}
