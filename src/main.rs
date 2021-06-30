mod cfg;

mod cli;
use cli::types::BuoArgs;

pub mod prelude;
use prelude::*;

pub mod util;
use util::media::dispatch_meta_fn;

use clap::Clap;

fn main() -> Result<()> {
    let BuoArgs {
        mut target_files,
        json,
    } = BuoArgs::parse();

    if target_files.is_empty() {
        bail!("No target files provided!");
    }

    // temporary restriction for testing
    assert_eq!(target_files.len(), 1);
    let target_file = target_files.pop().unwrap();

    // all directories get same treatment, dynamic dispatch not needed
    if target_file.is_dir() {
        if json {
            let dir_meta_json = util::dirs::serialized_dir_meta(&target_file)?;
            println!("{}", dir_meta_json);
            return Ok(());
        }

        let dir_meta = util::dirs::get_dir_meta(&target_file)?;
        println!("{}", dir_meta);
        return Ok(());
    }

    if let Some(dispatcher) = dispatch_meta_fn(&target_file) {
        // FileExtCallback found, dynamically dispatching
        let file_meta = dispatcher.try_get_meta(&target_file)?;

        let display_value = if json {
            // json serialized
            file_meta.as_json()?
        } else if file_meta.display_extra {
            // prints with unknown extra attrs
            file_meta.to_detailed_string()
        } else {
            file_meta.to_string()
        };

        println!("{}", display_value);
    } else {
        // No filetype associated callback found

        let file_type = target_file
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown");
        println!("Filetype not supported: {}", file_type);
    }

    Ok(())
}
