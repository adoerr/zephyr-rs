use argh::FromArgs;

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

#[derive(FromArgs, PartialEq, Debug)]
/// Generate Rust bindings from Zephyr headers
#[argh(subcommand, name = "bindgen")]
struct Bindgen {
    #[argh(option)]
    /// header file with Zephyr definitions
    header: String,
}

fn main() {
    let cli: CLI = argh::from_env();

    match cli.command {
        Command::Bindgen(bindgen) => {
            println!("bindgen: {:?}", bindgen);
        }
    }
}
