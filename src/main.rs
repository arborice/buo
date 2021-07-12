mod cli;
use cli::{dispatch_from_cli, fetch_cli_args};

mod prelude;
use prelude::*;

mod util;

fn main() -> Result<()> {
    let buo_args = fetch_cli_args()?;
    dispatch_from_cli(buo_args)
}
