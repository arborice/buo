use crate::prelude::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// NOT FINAL
#[derive(Serialize)]
pub struct DirMeta {
    pub path: PathBuf,
    // will eventually be u64, but right now it is the output from dust subprocess
    pub disk_size: String,
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

/// Temporary function to calculate the size of a directory from dust binary.
/// `dust` has no lib, so will implement here later w/ rayon acceleration
fn get_dust_output(path: &Path) -> Result<String> {
    let dust_stdout = Command::new("dust")
        .args(&["-bcrn", "1"])
        .arg(path)
        .output()?;

    let stringified_output = String::from_utf8(dust_stdout.stdout)?;
    Ok(stringified_output)
}

use std::fs::read_dir;
fn dirty_file_count(path: &Path, count: &mut u64) -> Result<()> {
    for file in read_dir(path)? {
        let path = file?.path();
        if path.is_dir() {
            dirty_file_count(&path, count)?;
        }
        *count += 1;
    }
    Ok(())
}

/// counts number of files recursively
/// might be parallelized if all directories are filtered first?
fn calc_num_files(path: &Path) -> Result<u64> {
    let mut file_count = 0;
    dirty_file_count(path, &mut file_count)?;
    Ok(file_count)
}

fn parse_dust_size(dust_stdout: &str) -> Option<String> {
    dust_stdout.split_whitespace().next().map(|s| s.to_string())
}

/// only temporary layout for DirMeta struct.
/// right now depends on `dust`
pub fn get_dir_meta(dir_path: &Path) -> Result<DirMeta> {
    assert!(dir_path.is_dir());

    let dust_stdout = get_dust_output(dir_path)?;
    let disk_size = parse_dust_size(&dust_stdout).ok_or_else(|| {
        dbg!("invalid output:\n{}\n", dust_stdout);
        anyhow!("Dust parsing error!")
    })?;

    let num_files = calc_num_files(dir_path)?;

    Ok(DirMeta {
        path: dir_path.to_path_buf(),
        disk_size,
        num_files,
    })
}
