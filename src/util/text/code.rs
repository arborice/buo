use crate::{
    prelude::*,
    util::{
        dev::{collect_into_stats, get_readme_contents_if_exists},
        media::meta::{IntoMeta, MediaMeta},
        traits::ExtCallback,
    },
};
use tokei::{Config, Languages};

pub struct CodeAnalyzer;
impl ExtCallback for CodeAnalyzer {
    fn try_get_meta(&self, path: &std::path::Path) -> Result<MediaMeta> {
        if !path.exists() {
            bail!("{} does not exist!", path.display());
        }

        let mut path = path.to_path_buf();
        path.pop();

        let tokei_cfg = Config::default();
        let mut tokei_instance = Languages::new();

        tokei_instance.get_statistics(&[&path], &[], &tokei_cfg);
        let mut media_meta: MediaMeta =
            tokei_instance.into_meta(path.to_string_lossy().to_string())?;

        if let Some(readme) = get_readme_contents_if_exists(&path) {
            let readme_contents = format!("Readme:\n{}", readme?);
            media_meta.extra.replace(readme_contents);
            media_meta.display_extra = true;
        }
        Ok(media_meta)
    }
}

impl IntoMeta for Languages {
    fn into_meta(self, file_name: String) -> Result<MediaMeta> {
        let summary = self.total();
        let stats = collect_into_stats(summary.children);

        Ok(MediaMeta {
            file_name,
            stats: Some(stats),
            ..Default::default()
        })
    }
}
