mod bindgen;

use anyhow::Result;
use argh::FromArgs;

use crate::bindgen::{generate, Bindgen};

#[derive(FromArgs)]
#[allow(clippy::upper_case_acronyms)]
/// xtask
struct CLI {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    Bindgen(Bindgen),
}

fn main() -> Result<()> {
    let cli: CLI = argh::from_env();

    match cli.command {
        Command::Bindgen(cfg) => generate(cfg)?,
    };

    Ok(())
}
