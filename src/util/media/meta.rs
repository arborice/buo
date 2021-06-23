use crate::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Default, Serialize, Deserialize)]
pub struct MediaMeta {
    pub file_name: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub duration: Option<std::time::Duration>,
    pub date: Option<DateKind>,
    pub extra: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum DateKind {
    Chrono(DateTime<Utc>),
    Sym(String),
}

pub trait IntoMeta {
    fn into_meta(self, file_name: String) -> Result<MediaMeta>;
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

impl fmt::Display for MediaMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = format!("file name: {}\n", &self.file_name);
        if let Some(ref title) = self.title {
            out += &format!("title: {}\n", title);
        }

        if let Some(ref author) = self.author {
            out += &format!("author: {}\n", author);
        }

        if let Some(ref duration) = self.duration {
            out += &format!("duration: {:?}\n", duration);
        }

        if let Some(ref date) = self.date {
            out += &date.to_string();
        }
        write!(f, "{}", out)
    }
}

impl MediaMeta {
    pub fn to_detailed_string(&self) -> String {
        self.to_string() + self.extra.as_ref().unwrap_or(&"".into())
    }
}
