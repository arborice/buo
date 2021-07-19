use crate::prelude::*;
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Serialize, strum::Display)]
pub enum ExportKind {
    Dir,
    File,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportedJson<T>
where
    T: Serialize + fmt::Display,
{
    file_type: ExportKind,
    date: DateTime<Utc>,
    #[serde(flatten)]
    inner: T,
}

use super::media::meta::MediaMeta;
impl From<MediaMeta> for ExportedJson<MediaMeta> {
    fn from(media_meta: MediaMeta) -> Self {
        ExportedJson {
            file_type: ExportKind::File,
            date: Utc::now(),
            inner: media_meta,
        }
    }
}

use super::dirs::DirMeta;
impl From<DirMeta> for ExportedJson<DirMeta> {
    fn from(dir_meta: DirMeta) -> Self {
        ExportedJson {
            file_type: ExportKind::Dir,
            date: Utc::now(),
            inner: dir_meta,
        }
    }
}

impl<T> ExportedJson<T>
where
    T: Serialize + fmt::Display,
{
    pub fn as_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn as_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl<T> fmt::Display for ExportedJson<T>
where
    T: Serialize + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type: {}\n{}", self.file_type, self.inner)
    }
}
