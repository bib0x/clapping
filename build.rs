use clap_complete::{generate_to, shells::Bash};
use std::io::Error;

include!("src/cli/mod.rs");

fn main() -> Result<(), Error> {
    let outdir = "./completions";
    let pkgname = env!("CARGO_PKG_NAME");

    let mut cmd = build_cli();
    let path = generate_to(
        Bash,
        &mut cmd, // We need to specify what generator to use
        pkgname,  // We need to specify the bin name manually
        outdir,   // We need to specify where to write to
    )?;

    println!("cargo:warning=completion file is generated: {:?}", path);

    Ok(())
}
