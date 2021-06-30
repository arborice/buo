pub mod audio;
pub mod meta;
pub mod video;

use crate::{
    prelude::*,
    util::{text::code::CodeAnalyzer, traits::ExtCallback},
};
use std::path::Path;
use strum::IntoEnumIterator;

pub struct FileExtCallback(FileExt, &'static dyn ExtCallback);

struct DefaultAnalyzer;
impl ExtCallback for DefaultAnalyzer {
    fn try_get_meta(&self, _path: &Path) -> Result<meta::MediaMeta> {
        unreachable!("dummy impl")
    }
}

static DA: &DefaultAnalyzer = &DefaultAnalyzer;
impl Default for FileExtCallback {
    fn default() -> Self {
        Self(FileExt::default(), DA)
    }
}

type DynExtMap<const LEN: usize> = [FileExtCallback; LEN];

use once_cell::sync::Lazy;
use tinyvec::{array_vec, ArrayVec};
pub static EXT_FNS: Lazy<ArrayVec<DynExtMap<3>>> = Lazy::new(|| {
    let mut tied = array_vec!(DynExtMap<3>);

    static AA: &audio::AudioAnalyzer = &audio::AudioAnalyzer;
    static CA: &CodeAnalyzer = &CodeAnalyzer;
    static VA: &video::VideoAnalyzer = &video::VideoAnalyzer;

    for ext in FileExt::iter() {
        if ext.is_audio() {
            tied.push(FileExtCallback(ext, AA));
        } else if ext.is_video() {
            tied.push(FileExtCallback(ext, VA));
        } else if ext.is_dev() {
            tied.push(FileExtCallback(ext, CA));
        }
    }

    if tied
        .iter()
        .any(|FileExtCallback(ext, _)| *ext == FileExt::default())
    {
        panic!("EXT_FNS LEN incorrect");
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
