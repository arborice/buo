use crate::prelude::*;
use rusqlite::{Connection, OpenFlags, Row};

use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use super::WebBookmark;

const FF_ROOT_PATH: &str = ".mozilla/firefox";
const PLACES: &str = "places.sqlite";
const QUERY: &str = "SELECT b.title, p.url FROM moz_bookmarks b JOIN moz_places p ON p.id = b.fk";

fn with_default_firefox_profile(firefox_dir: &Path) -> Result<PathBuf> {
    for ent in read_dir(firefox_dir)? {
        let path = ent?.path();
        if path.to_string_lossy().ends_with("default-release") {
            return Ok(path);
        }
    }
    bail!("No default firefox profile available")
}

fn query_bookmarks_from_path(db_path: &Path) -> Result<Vec<WebBookmark>> {
    let db = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut query = db.prepare(QUERY)?;

    let bookmarks = query
        .query_map([], |row| {
            Ok(WebBookmark {
                name: row.get(0)?,
                url: row.get(1)?,
            })
        })?
        .filter_map(|b| b.ok())
        .collect();

    Ok(bookmarks)
}

pub fn get_firefox_bookmarks(cfg_path: &Path) -> Result<Vec<WebBookmark>> {
    let mut ff_path = cfg_path.to_path_buf();
    ff_path.push(FF_ROOT_PATH);

    let mut db_path = with_default_firefox_profile(&ff_path)?;
    db_path.push(PLACES);
    query_bookmarks_from_path(&db_path)
}

#[test]
fn firefox_bookmarks() -> Result<()> {
    let bookmarks_path = dirs::home_dir().ok_or_else(|| anyhow!("no home dir"))?;
    let bookmarks = get_firefox_bookmarks(&bookmarks_path)?;

    for bkm in bookmarks {
        println!("{:?}", bkm);
    }
    Ok(())
}
