use crate::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct DirMeta {
    pub path: PathBuf,
    pub disk_size: u64,
    // this may be able to be parallelized
    pub num_files: u64,
}

use std::fmt;
impl fmt::Display for DirMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "path: {}\n{} files\ndisk size: <{}>",
            self.path.display(),
            &self.num_files,
            &self.disk_size
        )
    }
}

#[derive(Default)]
struct DirWalker {
    total: u64,
    dir_ent_count: u64,
    io_err_count: u16,
    filesize_err_count: u16,
}

use filesize::PathExt;
use std::fs::read_dir;
fn recurse_total_dir_size(dir_path: &Path, walk_results: &mut DirWalker) {
    if let Ok(walker) = read_dir(dir_path) {
        for dir_ent in walker {
            if let Ok((path, Ok(meta))) = dir_ent.map(|e| (e.path(), e.metadata())) {
                if path.is_dir() {
                    recurse_total_dir_size(&path, walk_results);
                }

                if let Ok(file_disk_size) = path.size_on_disk_fast(&meta) {
                    walk_results.total += file_disk_size;
                    walk_results.dir_ent_count += 1;
                } else {
                    walk_results.filesize_err_count += 1;
                }
            } else {
                walk_results.io_err_count += 1;
            }
        }
    } else {
        walk_results.io_err_count += 1;
    }
}

/// only temporary layout for DirMeta struct.
pub fn get_dir_meta(dir_path: &Path) -> Result<DirMeta> {
    assert!(dir_path.is_dir());

    let mut dir_walker = DirWalker::default();
    recurse_total_dir_size(dir_path, &mut dir_walker);

    if dir_walker.io_err_count > 0 {
        eprintln!("{} io errors encountered.", dir_walker.io_err_count);
    }

    if dir_walker.filesize_err_count > 0 {
        eprintln!(
            "{} filesize crate errors encountered.",
            dir_walker.filesize_err_count
        );
    }

    Ok(DirMeta {
        path: dir_path.to_path_buf(),
        disk_size: dir_walker.total,
        num_files: dir_walker.dir_ent_count,
    })
}
