mod cfg;

mod cli;
use cli::types::BuoArgs;

pub mod prelude;
use prelude::*;

pub mod util;
use util::media::dispatch_meta_fn;

use clap::Clap;

fn main() -> Result<()> {
    let BuoArgs { mut target_files } = BuoArgs::parse();

    if target_files.is_empty() {
        bail!("No target files provided!");
    }

    for target_file in target_files.drain(..) {
        if let Some(dispatcher) = dispatch_meta_fn(&target_file) {
            let file_meta = dispatcher.try_get_meta(&target_file)?;

            let display_value = if file_meta.display_extra {
                file_meta.to_detailed_string()
            } else {
                file_meta.to_string()
            };
            println!("{}", display_value);
        } else {
            let file_type = target_file
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown");
            println!("Filetype not supported: {}", file_type);
        }
    }
    Ok(())
}
