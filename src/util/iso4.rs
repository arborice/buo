use crate::prelude::*;
use std::{fs::File, path::Path};

use symphonia::core::{
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::{MetadataOptions, StandardTagKey, Tag, Value},
    probe::{Hint, ProbeResult},
};

#[inline]
pub fn iso4_meta(path: &Path) -> Result<MediaMeta> {
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
    iso4_media_meta(file_name, probe)
}

fn iso4_media_meta(file_name: String, probe: ProbeResult) -> Result<MediaMeta> {
    if let Some(meta) = probe
        .format
        .metadata()
        .current()
        .or_else(|| probe.metadata.current())
    {
        meta.tags().into_meta(file_name)
    } else {
        Ok(MediaMeta::with_file_name(file_name))
    }
}

/// Replaces the T inside the tag if the stringified value is not empty.
/// The functor makes this a more flexible parser
fn replace_tag_if_not_empty<T>(tag_value: &Value, tag: &mut Option<T>, functor: fn(String) -> T) {
    let tag_value = tag_value.to_string();
    if !tag_value.is_empty() {
        tag.replace(functor(tag_value));
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
                StandardTagKey::Artist => replace_tag_if_not_empty(value, &mut author, |a| a),
                StandardTagKey::AlbumArtist | StandardTagKey::Composer | StandardTagKey::Writer => {
                    if author.is_none() {
                        replace_tag_if_not_empty(value, &mut author, |a| a);
                    }
                }
                StandardTagKey::OriginalDate => {
                    replace_tag_if_not_empty(value, &mut date, DateKind::Sym)
                }
                StandardTagKey::TrackTitle => replace_tag_if_not_empty(value, &mut title, |t| t),
                _ => {}
            }
        }

        let maybe_extra = unknown
            .drain(..)
            .map(|Tag { key, value, .. }| format!("{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("\n");
        let extra = if maybe_extra.is_empty() {
            None
        } else {
            Some(maybe_extra)
        };

        Ok(MediaMeta {
            file_name,
            title,
            author,
            date,
            extra,
            ..Default::default()
        })
    }
}
