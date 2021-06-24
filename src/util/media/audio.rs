use crate::prelude::*;
use std::fs::File;

use symphonia::core::{
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::{MetadataOptions, StandardTagKey, Tag},
    probe::{Hint, ProbeResult},
};

use crate::util::traits::ExtCallback;
pub struct AudioAnalyzer;

impl ExtCallback for AudioAnalyzer {
    fn try_get_meta(self, path: &std::path::Path) -> Result<MediaMeta> {
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

        let file_name = get_file_name(path);
        pretty_meta(file_name, probe)
    }
}

fn pretty_meta(file_name: String, probe: ProbeResult) -> Result<MediaMeta> {
    if let Some(meta) = probe
        .format
        .metadata()
        .current()
        .or_else(|| probe.metadata.current())
    {
        let tags = meta.tags();
        tags.into_meta(file_name)
    } else {
        Ok(MediaMeta::with_file_name(file_name))
    }
}

impl IntoMeta for &[Tag] {
    fn into_meta(self, file_name: String) -> Result<MediaMeta> {
        if self.is_empty() {
            return Err(anyhow!("No metadata available!"));
        }

        let (mut known, mut unknown): (Vec<_>, Vec<_>) =
            self.iter().partition(|tag| tag.is_known());

        let mut title = None;
        let mut author = None;
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
                StandardTagKey::OriginalDate => {
                    date.replace(DateKind::Sym(value.to_string()));
                }
                StandardTagKey::TrackTitle => {
                    title.replace(value.to_string());
                }
                _ => {}
            }
        }

        let extra = unknown
            .drain(..)
            .map(|Tag { key, value, .. }| format!("{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(MediaMeta {
            file_name,
            title,
            author,
            date,
            extra: Some(extra),
            ..Default::default()
        })
    }
}
