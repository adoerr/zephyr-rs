use std::{env, fs, path::PathBuf};

use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs)]
#[allow(clippy::upper_case_acronyms)]
/// xtask
struct CLI {
    #[argh(positional)]
    /// path to crate for which to generate bindings
    crate_path: PathBuf,
}

fn main() -> Result<()> {
    let cli: CLI = argh::from_env();

    let crate_path = fs::canonicalize(cli.crate_path)?;
    eprintln!("zephyr-bindgen: crate path `{}`", crate_path.display());

    let flags = env::var("TARGET_CFLAGS").unwrap_or("".to_string());
    eprintln!("zephyr-bindgen: cflags `{}`", flags);

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(crate_path.join("wrapper.h").to_str().unwrap())
        .use_core()
        .clang_args(flags.split(' '))
        .generate()?;

    bindings.write_to_file(crate_path.join("src/bindings.rs"))?;

    Ok(())
}
