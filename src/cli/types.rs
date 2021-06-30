use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub struct BuoArgs {
    #[clap(short, long)]
    pub json: bool,
    #[clap(name = "target_file")]
    pub target_files: Vec<PathBuf>,
}
