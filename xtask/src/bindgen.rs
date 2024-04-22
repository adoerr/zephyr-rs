use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Generate Rust bindings for the Zephyr RTOS.
#[argh(subcommand, name = "bindgen")]
pub struct Bindgen {
    #[argh(positional)]
    /// path to header file with Zephyr definitions
    header: PathBuf,

    #[argh(positional)]
    /// path to Zephyr's include directory
    inc_path: PathBuf,
}

pub fn generate(cfg: Bindgen) -> Result<()> {
    fs::canonicalize(&cfg.header)
        .map_err(|_| anyhow!("header `{:?}` not found", cfg.header))?
        .is_file()
        .then_some(())
        .ok_or_else(|| anyhow!("header `{:?}` not found", cfg.header))?;

    fs::canonicalize(&cfg.inc_path)
        .map_err(|_| anyhow!("Zephyr include directory not found at {:?}", &cfg.inc_path))?
        .is_dir()
        .then_some(())
        .ok_or_else(|| {
            anyhow::anyhow!("Zephyr include directory not found at {:?}", &cfg.inc_path)
        })?;

    println!(
        "Generating bindings for {:?} in {:?}",
        cfg.header, cfg.inc_path
    );

    Ok(())
}
