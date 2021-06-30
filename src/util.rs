pub mod dev;
pub mod dirs;
pub mod file_types;
pub mod iso4;
pub mod media;
pub mod text;
pub mod traits;

use crate::prelude::*;
use std::fmt;

#[derive(Serialize, strum::Display)]
#[serde(rename_all = "lowercase")]
pub enum ExportKind {
    Dir,
    File,
}

#[derive(Serialize)]
pub struct ExportedJson<T>
where
    T: Serialize + fmt::Display,
{
    r#type: ExportKind,
    #[serde(flatten)]
    inner: T,
}

impl<T> ExportedJson<T>
where
    T: Serialize + fmt::Display,
{
    pub fn with_export_kind(inner: T, is_dir: bool) -> Self {
        let r#type = if is_dir {
            ExportKind::Dir
        } else {
            ExportKind::File
        };

        Self { r#type, inner }
    }

    pub fn pretty_print(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn print(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl<T> fmt::Display for ExportedJson<T>
where
    T: Serialize + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type: {}\n{}", self.r#type, self.inner)
    }
}
