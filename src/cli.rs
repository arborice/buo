pub mod args;
use args::BuoArgs;
use clap::Clap;

use crate::{
    prelude::*,
    util::{json_out::ExportedJson, media::dispatch_meta_fn},
};

pub fn fetch_cli_args() -> Result<BuoArgs> {
    let args = BuoArgs::parse();
    if args.target_files.is_empty() {
        bail!("No target files provided!")
    } else {
        Ok(args)
    }
}

fn print_cli_output<Meta>(meta: ExportedJson<Meta>, json: bool, prettify: bool) -> Result<()>
where
    Meta: Serialize + std::fmt::Display,
{
    let formatted_output = if prettify {
        meta.as_pretty_json()?
    } else if json {
        meta.as_json()?
    } else {
        meta.to_string()
    };
    println!("{}", formatted_output);
    Ok(())
}

pub fn dispatch_from_cli(
    BuoArgs {
        json,
        prettify,
        target_files,
    }: BuoArgs,
) -> Result<()> {
    for target_file in target_files {
        // all directories get same treatment, dynamic dispatch not needed
        if target_file.is_dir() {
            use super::util::dirs::*;

            let dir_meta = get_dir_meta(&target_file)?;
            let wrapped_meta: ExportedJson<_> = dir_meta.into();
            print_cli_output(wrapped_meta, json, prettify)?;
        } else if let Some(dispatcher) = dispatch_meta_fn(&target_file) {
            // FileExtCallback found, dynamically dispatching
            let file_meta = dispatcher.try_get_meta(&target_file)?;
            match file_meta {
                Some(meta) => {
                    let wrapped_meta: ExportedJson<_> = meta.into();
                    print_cli_output(wrapped_meta, json, prettify)?;
                }
                None => println!("No metadata for {}", target_file.display()),
            }
        } else {
            // No filetype associated callback found
            let file_type = target_file
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown");
            println!("Filetype not supported: {}", file_type);
        }
    }

    Ok(())
}
