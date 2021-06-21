use serde::Deserialize;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

#[derive(Debug, Deserialize)]
pub struct NodeMeta<'name> {
    pub ext: FileExt,
    pub file_name: &'name str,
}

#[derive(AsRefStr, Debug, Deserialize, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum FileExt {
    #[strum(serialize = "Not a supported file type")]
    Invalid,
    Mkv,
    Flv,
    Mp4,
    Mp3,
    M4v,
    M4a,
    Mov,

    // TEXT
    Txt,
    Docx,
    Md,

    Webm,
    Gif,
    Png,
    Jpeg,
    Jpg,
    #[strum(serialize = "RAW")]
    Raw,
    Html,
    Js,
    Rs,
    Go,
    Ts,
    Py,
    Cpp,
    C,
    Zig,
    Sh,
    Css,
    Dart,
    Java,
    Toml,
    Yaml,
    Yml,
    Json,
    Ini,
    Xss,
}

impl FileExt {
    const TEXT: &'static [Self] = &[Self::Txt, Self::Md, Self::Docx];
    pub fn is_text(&self) -> bool {
        Self::TEXT.contains(self)
    }

    const AUDIO: &'static [Self] = &[Self::Mp3, Self::M4a];
    pub fn is_audio(&self) -> bool {
        Self::AUDIO.contains(self)
    }

    const VIDEO: &'static [Self] = &[Self::Mkv, Self::Flv, Self::Mov, Self::Mp4, Self::Webm];
    pub fn is_video(&self) -> bool {
        Self::VIDEO.contains(self)
    }

    const IMG: &'static [Self] = &[Self::Png, Self::Jpg, Self::Jpeg, Self::Gif];
    pub fn is_img(&self) -> bool {
        Self::IMG.contains(self)
    }

    const DEV: &'static [Self] = &[
        Self::Html,
        Self::Js,
        Self::Rs,
        Self::Go,
        Self::Ts,
        Self::Py,
        Self::Cpp,
        Self::C,
        Self::Zig,
        Self::Sh,
        Self::Css,
        Self::Dart,
        Self::Java,
    ];
    pub fn is_dev(&self) -> bool {
        Self::DEV.contains(self)
    }

    const DATA: &'static [Self] = &[
        Self::Json,
        Self::Toml,
        Self::Yml,
        Self::Yaml,
        Self::Ini,
        Self::Xss,
    ];
    pub fn is_data(&self) -> bool {
        Self::DATA.contains(self)
    }
}

impl Default for FileExt {
    fn default() -> Self {
        Self::Invalid
    }
}

impl From<&str> for FileExt {
    fn from(s: &str) -> FileExt {
        FileExt::iter()
            .find(|ext| ext.as_ref() == s)
            .unwrap_or_default()
    }
}
