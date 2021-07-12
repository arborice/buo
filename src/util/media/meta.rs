use crate::{prelude::*, util::dev::LangStats};
use chrono::{DateTime, Utc};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct MediaMeta {
    pub file_name: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub duration: Option<std::time::Duration>,
    #[serde(rename = "media_date")]
    pub date: Option<DateKind>,
    pub stats: Option<Vec<LangStats>>,

    // if this option is enabled and extra is not empty,
    // display extra contents as well
    pub display_extra: bool,
    pub extra: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DateKind {
    Chrono(DateTime<Utc>),
    Sym(String),
}

use std::fmt;
impl fmt::Display for DateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Chrono(date) => {
                let formatted = date.format("%a %b %e %T %Y");
                writeln!(f, "date: {}", formatted)
            }
            Self::Sym(year) => writeln!(f, "year: {}", year),
        }
    }
}

macro_rules! append_metatag_if_not_empty {
    ($out:expr, $metatag:expr, $($tokens:tt)*) => {
        use colored::Colorize;
        if !$metatag.is_empty() {
            $out.push_str(&format!($($tokens)*, $metatag).green());
        }
    };
}

impl fmt::Display for MediaMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = format!("file name: {}\n", &self.file_name);
        if let Some(ref stats) = self.stats {
            out += &stats
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            return write!(f, "{}", out);
        }

        if let Some(ref title) = self.title {
            append_metatag_if_not_empty!(&mut out, title, "title: {}\n");
        }

        if let Some(ref author) = self.author {
            append_metatag_if_not_empty!(&mut out, author, "author: {}\n");
        }

        if let Some(ref duration) = self.duration {
            out += &format!("duration: {:?}\n", duration);
        }

        if let Some(ref date) = self.date {
            append_metatag_if_not_empty!(&mut out, date.to_string(), "{}");
        }
        write!(f, "{}", out)
    }
}

impl MediaMeta {
    pub fn with_file_name(file_name: String) -> Self {
        Self {
            file_name,
            ..Default::default()
        }
    }
}
