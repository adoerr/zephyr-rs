use std::{env, fs, path::PathBuf};

use anyhow::{Result};
use argh::FromArgs;
use bindgen::Builder;

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
    let header = fs::canonicalize(&cfg.header)?;

    let inc_path = fs::canonicalize(&cfg.inc_path)?;

    println!(
        "Generating bindings for {:?} in {:?}",
        cfg.header, cfg.inc_path
    );

    let bindings = Builder::default()
        .header(header.to_str().unwrap())
        .clang_arg(format!("-I{}", inc_path.to_str().unwrap()))
        .use_core()
        .generate()?;

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
