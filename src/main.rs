// mods
mod cli;
mod gui;
mod log;

// std
use std::{borrow::Borrow, path::PathBuf};

// libs
#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use cli::Cli;

#[allow(dead_code)]
fn defualt_deck_path() -> PathBuf{
    let mut pictures = dirs::picture_dir().unwrap();
    pictures.push("baraja");
    pictures
}

#[allow(dead_code)]
fn defualt_board_path() -> PathBuf{
    let mut pictures = dirs::picture_dir().unwrap();
    pictures.push("cartas");
    pictures
}

#[derive(Parser, Debug, Default, Clone)]
#[command(version = "0.4", author = "Daniel Alanis")]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(short = 'D', long)]
    debug: bool,

    #[clap(long)]
    override_config: Option<String>,
    #[clap( short, long, default_value = defualt_board_path().into_os_string() )]
    output: PathBuf,

    #[clap( short, long, default_value = defualt_deck_path().into_os_string() )]
    deck: PathBuf,

    #[clap(short, long, default_value = "image-")]
    in_format: String,

    #[clap(short, long, default_value = "carta-")]
    out_format: String,

    #[clap(subcommand)]
    option: Option<Instructions>,
}

#[derive(Subcommand, Debug, Default, Clone)]
enum Instructions{
    Cli{ path: PathBuf },
    Install,
    Update,
    #[default]
    Gui,
}

fn main() -> Result<()>{
    let opts = Opts::parse();
    let inst = opts.option.clone().unwrap_or(Instructions::Gui);
    debug!(opts, "options: {opts:?}");

    match inst {
        Instructions::Gui => gui::GUI::new(opts).run(),
        Instructions::Cli { path } => Cli::config(opts, path)?.run(),
        _ => Err(anyhow!("not implemented")),
    }?;

    Ok(())
}
