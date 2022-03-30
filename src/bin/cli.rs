use clap::Parser;
use std::{
    error,
    fs::File,
    io::{Read, Write},
};
use vscode2helix::converter;

/// Convert a vscode theme json to a helix theme toml
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The vscode theme input json file path
    #[clap(short, long)]
    input: String,

    /// The helix theme output toml file path
    #[clap(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let mut vs_file = File::open(&args.input)?;
    let mut vs_text = String::new();

    vs_file.read_to_string(&mut vs_text)?;

    let hx_text = converter::vscode2helix(&vs_text)?;

    let hx_filename = args.output.unwrap_or_else(|| {
        let name = args
            .input
            .clone()
            .strip_suffix(".jsonc")
            .unwrap_or_else(|| args.input.strip_suffix(".json").unwrap())
            .to_string();

        format!("{}.toml", name)
    });

    let mut hx_file = File::create(hx_filename)?;

    hx_file.write_all(hx_text.as_bytes())?;

    Ok(())
}
