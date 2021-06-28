pub mod audio;
pub mod meta;
pub mod video;

use super::text::code::CodeAnalyzer;
use std::{collections::HashMap, path::Path};
use strum::IntoEnumIterator;

use crate::util::traits::ExtCallback;
type DynExtMap = HashMap<FileExt, &'static dyn ExtCallback>;

use once_cell::sync::Lazy;
pub static EXT_FNS: Lazy<DynExtMap> = Lazy::new(|| {
    let mut tied: DynExtMap = HashMap::new();
    let all_exts: Vec<FileExt> = FileExt::iter().collect();

    static AA: &audio::AudioAnalyzer = &audio::AudioAnalyzer;
    static CA: &CodeAnalyzer = &CodeAnalyzer;
    static VA: &video::VideoAnalyzer = &video::VideoAnalyzer;

    for ext in all_exts {
        if ext.is_audio() {
            tied.insert(ext, AA);
        } else if ext.is_video() {
            tied.insert(ext, VA);
        } else if ext.is_dev() {
            tied.insert(ext, CA);
        }
    }

    tied
});

use super::file_types::*;
pub fn dispatch_meta_fn<'dispatch>(
    file_path: &'_ Path,
) -> Option<&'dispatch &'static dyn ExtCallback> {
    let file_ext: FileExt = file_path.extension()?.into();
    EXT_FNS.get(&file_ext)
}
