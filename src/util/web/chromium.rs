use super::WebBookmark;
use crate::{prelude::*, util::os::Os};
use serde_json::Value;
use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

pub enum ChromiumBrowser {
    Brave,
    Chrome,
    Chromium,
    Edge,
    Vivaldi,
}

fn with_default_profile(browser_dir: &mut PathBuf) -> Result<()> {
    browser_dir.push("Default");
    if browser_dir.exists() {
        return Ok(());
    }

    browser_dir.pop();
    for ent in read_dir(&browser_dir)? {
        let path = ent?.path();
        if path.to_string_lossy().starts_with("Profile") {
            browser_dir.push(path);
            break;
        }
    }
    Ok(())
}

pub fn resolve_chromium_browser_bookmark_path(browser: ChromiumBrowser, os: Os) -> Result<PathBuf> {
    use ChromiumBrowser::*;
    use Os::*;

    let mut bkmarks = dirs::config_dir().ok_or_else(|| anyhow!("Unable to locate config dir"))?;

    match (browser, os) {
        (Brave, Linux) => bkmarks.push("BraveSoftware/Brave-Browser/"),
        (Vivaldi, Linux) => bkmarks.push("vivaldi/"),
        (Brave, Windows) => bkmarks.push("Local/BraveSoftware/Brave-Browser/"),
        (Chrome, Windows) => bkmarks.push("Local/Google/Chrome/"),
        (Chromium, Windows) => bkmarks.push("Local/Google/Chromium/"),
        (Edge, Windows) => bkmarks.push("Local/Microsoft/Edge/User Data/"),
        (Vivaldi, Windows) => bkmarks.push("Local/Vivaldi/Application/"),
        _ => todo!(),
    };

    with_default_profile(&mut bkmarks)?;
    bkmarks.push("Bookmarks");
    Ok(bkmarks)
}

pub fn get_chromium_bookmarks(bookmark_path: &Path) -> Result<Option<Vec<WebBookmark>>> {
    let bookmark_file_contents = read_to_string(bookmark_path)?;
    let mut bookmarks_json: Value = serde_json::from_str(&bookmark_file_contents)?;

    if let Value::Object(mut root_folders) = bookmarks_json["roots"].take() {
        let keys: Vec<_> = root_folders.keys().map(|k| k.to_string()).collect();
        let mut bookmarks = vec![];

        for key in keys {
            // this is a safe unwrap
            let mut child = root_folders.remove(&key).unwrap();
            let child_bookmarks =
                serde_json::from_value::<Vec<WebBookmark>>(child["children"].take());

            match child_bookmarks {
                Ok(mut bkmks) => bookmarks.extend_from_slice(bkmks.as_mut_slice()),
                Err(e) if e.is_data() => continue,
                Err(e) => bail!(e),
            }
        }

        match bookmarks.len() {
            0 => Ok(None),
            _ => Ok(Some(bookmarks)),
        }
    } else {
        Ok(None)
    }
}

#[test]
fn brave_bookmarks() -> Result<()> {
    let bookmarks_path = resolve_chromium_browser_bookmark_path(ChromiumBrowser::Brave, Os::Linux)?;
    let bookmarks = get_chromium_bookmarks(&bookmarks_path)?;

    match bookmarks {
        Some(bkmarks) => {
            for bkm in bkmarks {
                println!("{:?}", bkm);
            }
        }
        _ => println!("No bookmarks available"),
    }
    Ok(())
}

#[test]
fn vivaldi_bookmarks() -> Result<()> {
    let bookmarks_path =
        resolve_chromium_browser_bookmark_path(ChromiumBrowser::Vivaldi, Os::Linux)?;
    let bookmarks = get_chromium_bookmarks(&bookmarks_path)?;

    match bookmarks {
        Some(bkmarks) => {
            for bkm in bkmarks {
                println!("{:?}", bkm);
            }
        }
        _ => println!("No bookmarks available"),
    }
    Ok(())
}
