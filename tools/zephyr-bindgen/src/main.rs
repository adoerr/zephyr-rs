use std::{env, path::PathBuf};

use anyhow::Result;

fn main() -> Result<()> {
    let flags = env::var("TARGET_CFLAGS").unwrap_or("".to_string());
    eprintln!("cflags: {}", flags);

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .use_core()
        .clang_args(flags.split(' '))
        .generate()?;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
