pub mod audio;
pub mod meta;
pub mod video;

use crate::{
    prelude::*,
    util::{text::code::CodeAnalyzer, traits::ExtCallback},
};
use std::path::Path;
use strum::IntoEnumIterator;

#[derive(Clone, Copy)]
pub struct FileExtCallback(FileExt, &'static dyn ExtCallback);

struct DefaultAnalyzer;
impl ExtCallback for DefaultAnalyzer {
    fn try_get_meta(&self, _path: &Path) -> Result<Option<MediaMeta>> {
        unreachable!("dummy impl")
    }
}

static DA: &DefaultAnalyzer = &DefaultAnalyzer;
impl Default for FileExtCallback {
    fn default() -> Self {
        Self(FileExt::default(), DA)
    }
}

const LEN: usize = 32;
type DynExtMap = [FileExtCallback; LEN];

use once_cell::sync::Lazy;
use tinyvec::{array_vec, ArrayVec};
pub static EXT_FNS: Lazy<ArrayVec<DynExtMap>> = Lazy::new(|| {
    static AA: &audio::AudioAnalyzer = &audio::AudioAnalyzer;
    static CA: &CodeAnalyzer = &CodeAnalyzer;
    static VA: &video::VideoAnalyzer = &video::VideoAnalyzer;

    let mut tied = array_vec!(DynExtMap);
    for ext in FileExt::iter() {
        if ext.is_audio() {
            tied.push(FileExtCallback(ext, AA));
        } else if ext.is_video() {
            tied.push(FileExtCallback(ext, VA));
        } else if ext.is_dev() {
            tied.push(FileExtCallback(ext, CA));
        }
    }

    tied
});

use super::file_types::*;
pub fn dispatch_meta_fn<'dispatch>(
    file_path: &'_ Path,
) -> Option<&'dispatch &'static dyn ExtCallback> {
    let file_ext: FileExt = file_path.extension()?.into();
    EXT_FNS
        .iter()
        .find(|FileExtCallback(ext, _)| *ext == file_ext)
        .map(|FileExtCallback(_, fun)| fun)
}
