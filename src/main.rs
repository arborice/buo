mod cfg;

mod cli;
use cli::types::BuoArgs;

pub mod prelude;
use prelude::*;

pub mod util;

use clap::Clap;

fn main() -> Result<()> {
    let BuoArgs { target_files } = BuoArgs::parse();

    for _ in target_files {
        // println!("{}", audio_meta);
    }
    Ok(())
}
