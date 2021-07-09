use crate::prelude::*;
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};
use tokei::{LanguageType, Report};

const README: &str = "README";

pub fn get_readme_contents_if_exists(parent_path: &Path) -> Option<Result<String>> {
    match read_dir(parent_path) {
        Ok(dir_ents) => {
            for file in dir_ents.filter_map(|f| f.ok()) {
                if file.file_name().to_string_lossy().contains(README) {
                    let read_res = read_to_string(file.path()).map_err(|e| anyhow!(e));
                    return Some(read_res);
                }
            }
        }
        Err(e) => return Some(Err(anyhow!(e))),
    }
    None
}

use std::num::NonZeroUsize;
#[derive(Clone, Deserialize, Serialize)]
pub struct LangStats {
    name: String,
    loc: Option<NonZeroUsize>,
    comments: Option<NonZeroUsize>,
}

use std::fmt;
impl fmt::Display for LangStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = self.name.clone() + "\n";

        match (self.loc, self.comments) {
            (Some(loc), Some(comments)) => {
                res += &format!("{} loc, {} comments.", loc, comments);
            }
            (Some(loc), None) => {
                res += &format!("{} loc.", loc);
            }
            (None, Some(comments)) => {
                res += &format!("{} comments.", comments);
            }
            _ => {}
        }

        write!(f, "{}", res)
    }
}

fn get_loc(report: &[Report]) -> usize {
    report.iter().map(|r| r.stats.code).sum()
}

fn get_comments(report: &[Report]) -> usize {
    report.iter().map(|r| r.stats.comments).sum()
}

use std::collections::BTreeMap;
pub fn collect_into_stats(lang_children: BTreeMap<LanguageType, Vec<Report>>) -> Vec<LangStats> {
    lang_children
        .iter()
        .map(|(lang, reports)| {
            let name = lang.name().to_owned();
            let loc = NonZeroUsize::new(get_loc(reports));
            let comments = NonZeroUsize::new(get_comments(reports));

            LangStats {
                name,
                loc,
                comments,
            }
        })
        .collect()
}
