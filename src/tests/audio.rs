use crate::prelude::*;
use std::{fs::File, path::Path};

use symphonia::core::{
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::{MetadataOptions, Tag, Value},
    probe::{Hint, ProbeResult},
};

pub fn get_audio_metadata(path: &Path) -> Result<Option<String>> {
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

fn pretty_tags(tags: &[Tag]) -> Option<String> {
    if tags.is_empty() {
        return None;
    }

    let (known, unknown): (Vec<_>, Vec<_>) = tags.iter().partition(|tag| tag.is_known());

    let mut prettified_tags = vec![];

    for known_tag in known {
        if let Some(key) = known_tag.std_key {
            prettified_tags.push(pretty_print_tag_item(
                &format!("{:?}", key),
                &known_tag.value,
            ));
        }
    }

    for unknown_tag in unknown {
        prettified_tags.push(pretty_print_tag_item(&unknown_tag.key, &unknown_tag.value));
    }

    Some(prettified_tags.join("\n"))
}

const fn indent() -> usize {
    4 + 28 + 1
}

fn pretty_print_tag_item(key: &str, value: &Value) -> String {
    let key_str = match key.len() {
        0..=28 => format!("{:<28} : ", key),
        _ => format!("{:.<28} : ", key.split_at(26).0,),
    };

    let line_prefix = format!("\n{:w$} : ", "", w = indent());
    let line_wrap_prefix = format!("\n{:w$}   ", "", w = indent());

    let mut out = key_str;

    for (wrapped, line) in value.to_string().lines().enumerate() {
        if wrapped > 0 {
            out += &line_prefix;
        }

        let mut chars = line.chars();
        let split = (0..)
            .map(|_| chars.by_ref().take(72).collect::<String>())
            .take_while(|s| !s.is_empty())
            .collect::<Vec<_>>();

        out.push_str(&split.join(&line_wrap_prefix));
    }

    out
}
