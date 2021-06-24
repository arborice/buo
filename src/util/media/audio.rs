use crate::{
    prelude::*,
    util::{iso4::iso4_meta, traits::ExtCallback},
};
pub struct AudioAnalyzer;

impl ExtCallback for AudioAnalyzer {
    fn try_get_meta(&self, path: &std::path::Path) -> Result<MediaMeta> {
        iso4_meta(path)
    }
}
