use serde::Deserialize;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

#[derive(AsRefStr, Copy, Clone, Debug, Deserialize, EnumIter, Hash, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum FileExt {
    #[strum(serialize = "Not a supported file type")]
    Invalid,

    // Audio
    Flac,
    Mp3,
    M4a,
    Ogg,
    Wav,

    // Video
    Mp4,
    M4v,
    Flv,
    Mkv,
    Mov,
    Webm,

    // Text
    Txt,
    Docx,
    Md,
    Odf,

    // Img
    Gif,
    Png,
    Jpeg,
    Jpg,
    #[strum(serialize = "RAW")]
    Raw,

    // Dev
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
    Xml,
}

#[allow(unused)]
impl FileExt {
    const TEXT: &'static [Self] = &[Self::Txt, Self::Md, Self::Docx, Self::Odf];
    pub fn is_text(&self) -> bool {
        Self::TEXT.contains(self)
    }

    const AUDIO: &'static [Self] = &[Self::Flac, Self::Mp3, Self::M4a, Self::Ogg, Self::Wav];
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
        Self::Json,
        Self::Toml,
        Self::Yml,
        Self::Yaml,
        Self::Ini,
        Self::Xml,
    ];
    pub fn is_dev(&self) -> bool {
        Self::DEV.contains(self)
    }

    pub fn is_iso4(&self) -> bool {
        use FileExt::*;
        matches!(self, Mp3 | Mp4 | M4v | M4a | Wav | Flac | Ogg)
    }

    pub fn is_matroska(&self) -> bool {
        matches!(self, Self::Mkv | Self::Webm)
    }
}

impl Default for FileExt {
    fn default() -> Self {
        Self::Invalid
    }
}

impl PartialEq<str> for FileExt {
    fn eq(&self, other: &str) -> bool {
        FileExt::iter().any(|ext| ext.as_ref() == other)
    }
}

// THESE ARE INFALLIBLE - NOT SUITABLE FOR DISPATCH
impl From<&str> for FileExt {
    fn from(s: &str) -> Self {
        FileExt::iter()
            .find(|ext| ext.as_ref() == s)
            .unwrap_or_default()
    }
}

impl From<&std::ffi::OsStr> for FileExt {
    fn from(s: &std::ffi::OsStr) -> Self {
        FileExt::iter()
            .find(|ext| ext.as_ref() == s)
            .unwrap_or_default()
    }
}
