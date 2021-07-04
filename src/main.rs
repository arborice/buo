mod cfg;

mod cli;
use cli::types::BuoArgs;

mod prelude;
use prelude::*;

mod util;
use util::media::dispatch_meta_fn;

use clap::Clap;
use util::ExportedJson;

fn main() -> Result<()> {
    let BuoArgs {
        mut target_files,
        json,
        prettify,
    } = BuoArgs::parse();

    if target_files.is_empty() {
        bail!("No target files provided!");
    }

    // temporary restriction for testing
    assert_eq!(target_files.len(), 1);
    let target_file = target_files.pop().unwrap();

    // all directories get same treatment, dynamic dispatch not needed
    if target_file.is_dir() {
        use util::dirs::*;

        let dir_meta = get_dir_meta(&target_file)?;
        let wrapped_meta: ExportedJson<_> = dir_meta.into();

        let output = if prettify {
            wrapped_meta.pretty_print()?
        } else if json {
            wrapped_meta.print()?
        } else {
            wrapped_meta.to_string()
        };

        println!("{}", output);
        return Ok(());
    }

    if let Some(dispatcher) = dispatch_meta_fn(&target_file) {
        // FileExtCallback found, dynamically dispatching
        let file_meta = dispatcher.try_get_meta(&target_file)?;
        let wrapped_meta: ExportedJson<_> = file_meta.into();

        let output = if prettify {
            // pretty print serialized json
            wrapped_meta.pretty_print()?
        } else if json {
            // json serialized
            wrapped_meta.print()?
        } else {
            wrapped_meta.to_string()
        };

        println!("{}", output);
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
