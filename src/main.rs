mod cfg;

mod cli;
use cli::types::BuoArgs;

pub mod prelude;
use prelude::*;

pub mod util;
use util::media::audio::get_audio_metadata;

use clap::Clap;

fn main() -> Result<()> {
    let BuoArgs { target_file } = BuoArgs::parse();

    if let Some(audio_meta) = get_audio_metadata(&target_file)? {
        println!("{}", audio_meta);
    } else {
        println!("No metadata found!");
    }

    Ok(())
}
