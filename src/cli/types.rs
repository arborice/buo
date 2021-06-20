use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub struct BerryArgs {
    #[clap(short, long)]
    pub target_file: PathBuf,
}
