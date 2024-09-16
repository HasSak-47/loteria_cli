// mods
mod cli;
mod gui;

// std
use std::{fs::File, io::{BufReader, Read}, path::PathBuf, str::FromStr};

// libs
#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use cli::Cli;

#[allow(dead_code)]
fn defualt_deck_path() -> PathBuf{
    let mut pictures = dirs::picture_dir().unwrap();
    pictures.push("barajas");
    pictures
}

#[allow(dead_code)]
fn defualt_board_path() -> PathBuf{
    let mut pictures = dirs::picture_dir().unwrap();
    pictures.push("cartas");
    pictures
}

#[derive(Parser, Debug)]
#[command(version = "0.4", author = "Daniel Alanis")]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(short = 'D', long)]
    debug: bool,

    #[clap(long)]
    override_config: Option<String>,

    #[clap(short, long, default_value = "defualt_board_path")]
    output: Option<PathBuf>,

    #[clap(short, long, default_value = "defualt_deck_path")]
    deck: Option<PathBuf>,

    #[clap(subcommand)]
    option: Option<Instructions>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DataOpts{
    verbose: bool,
    debug: bool,
}

impl From<&Opts> for DataOpts{
    fn from(value: &Opts) -> Self {
        Self{ verbose: value.verbose, debug: value.debug }
    }
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
    let dataops = DataOpts::from(&opts);

    match opts.option.unwrap_or( Instructions::default() ){
        Instructions::Gui => gui::GUI::new(dataops).run(),
        Instructions::Cli { path } => Cli::from_path(path)?.run(),
        _ => Err(anyhow!("not implemented")),
    }?;

    Ok(())
}
