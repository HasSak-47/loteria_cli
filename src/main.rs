// mods
mod gui;


// std
use std::path::PathBuf;

// libs
#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "0.4", author = "Daniel Alanis")]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(long)]
    override_config: Option<String>,

    #[clap(short, long)]
    output: Option<PathBuf>,

    #[clap(short, long)]
    deck: Option<PathBuf>,

    #[clap(subcommand)]
    option: Option<Instructions>,
}

#[derive(Subcommand, Debug, Clone)]
enum Instructions{
    File{ path: PathBuf },
    Args{ args: Vec<String> },
    Install,
    Update,
    Gui,
}

fn main() -> Result<()>{
    let _opts = Opts::parse();

    Ok(())
}
