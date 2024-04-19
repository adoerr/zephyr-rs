use std::path::PathBuf;

use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Generate Rust bindings for the Zephyr RTOS.
#[argh(subcommand, name = "bindgen")]
pub struct Bindgen {
    #[argh(positional)]
    /// header file with Zephyr definitions
    header: String,

    #[argh(positional)]
    /// path to Zephyr's include directory
    inc_path: PathBuf,
}

pub fn generate(_cfg: Bindgen) -> Result<()> {
    Ok(())
}
