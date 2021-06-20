mod cfg;

mod cli;
use cli::types::BerryArgs;

pub mod prelude;
use prelude::*;

mod tests;
use tests::audio::get_audio_metadata;

use clap::Clap;

fn main() -> Result<()> {
    let berry_args = BerryArgs::parse();
    let target_file = berry_args.target_file;

    let audio_meta = get_audio_metadata(&target_file)?;
    println!("{}", audio_meta);

    Ok(())
}
