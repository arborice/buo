use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReadCfg<'cfg> {
    pub web_browser: &'cfg str,
    pub include_browser_bookmarks: bool,
}

#[derive(Serialize)]
pub struct WriteCfg {
    pub web_browser: String,
    pub include_browser_bookmarks: bool,
}

#[derive(Deserialize, Serialize)]
pub struct CustomBookmark {
    pub path: std::path::PathBuf,
    // pub custom_reader:
}
