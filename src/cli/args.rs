use clap::{AppSettings, Clap};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct BuoArgs {
    /// output serialized json
    #[clap(short, long)]
    pub json: bool,
    /// prettify json output
    #[clap(short, long)]
    pub prettify: bool,
    #[clap(name = "target_file")]
    pub target_files: Vec<PathBuf>,
}
