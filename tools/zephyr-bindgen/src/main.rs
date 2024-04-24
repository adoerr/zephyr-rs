use std::{fs, path::PathBuf};

use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs)]
#[allow(clippy::upper_case_acronyms)]
/// zephyr-bindgen: generate Rust bindings for Zephyr
struct CLI {
    #[argh(positional)]
    /// path to crate for which to generate bindings
    crate_path: PathBuf,

    #[argh(positional, greedy)]
    /// cflags to pass to clang
    cflags: Vec<String>,
}

fn main() -> Result<()> {
    let cli: CLI = argh::from_env();

    let crate_path = fs::canonicalize(cli.crate_path)?;
    eprintln!("zephyr-bindgen: crate path `{}`", crate_path.display());

    let cflags_str = cli.cflags.join(" ");

    let bindings = bindgen::Builder::default()
        .header(crate_path.join("wrapper.h").to_str().unwrap())
        .allowlist_item("k_*")
        .use_core()
        .clang_args(cflags_str.split_whitespace())
        .generate()?;

    bindings.write_to_file(crate_path.join("src/bindings.rs"))?;

    Ok(())
}
