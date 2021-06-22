use crate::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Default, Serialize, Deserialize)]
pub struct MediaMeta {
    pub title: Option<String>,
    pub author: Option<String>,
    pub duration: Option<std::time::Duration>,
    pub date: Option<DateTime<Utc>>,
    pub extra: Option<String>,
}

use std::fmt;
impl fmt::Display for MediaMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
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
            let formatted = date.format("%a %b %e %T %Y");
            out += &format!("date: {}\n", formatted);
        }

        if let Some(ref extra) = self.extra {
            out += extra;
        }
        write!(f, "{}", out)
    }
}
