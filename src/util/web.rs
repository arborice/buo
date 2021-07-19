pub mod chromium;
pub mod firefox;

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebBookmark {
    pub name: Option<String>,
    pub url: String,
}
