use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub struct BuoArgs {
    #[clap(name = "target_file")]
    pub target_files: Vec<PathBuf>,
}
