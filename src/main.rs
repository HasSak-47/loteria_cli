mod cli;
mod installer;
mod utils;
mod config;

use std::{fs::read_dir, path::PathBuf};

#[allow(unused_imports)]
use anyhow::{anyhow, Result};

use cli::{c_instructions, get_instructions, run};
use image::{open, RgbImage};
use loteria_engine::engine::Board;

use crate::{
    utils::{get_deck_path, get_board_path},
    installer::install
};

fn create_board(b: Board, cards: &Vec<RgbImage>) -> RgbImage{
    let width  = cards[0].width();
    let height = cards[0].height();
    let len = width * height;
    let mut img = RgbImage::new(4 * width, 4 * height);

    for ij in 0..16u32{
        let i = ij % 4;
        let j = ij / 4;

        let card = b[i as usize][j as usize].unpack() as usize;

        for iijj in 0..len{
            let ii = iijj % width;
            let jj = iijj / width;
let card_px = cards[card].get_pixel(ii, jj);
            *img.get_pixel_mut(i * width + ii, j * height + jj) = *card_px;
        }
    }

    img
}

fn get_images(path: PathBuf) -> Result<Vec<RgbImage>> {
    let mut v = Vec::new();
    let path = get_deck_path()?;
    let d = read_dir(path).unwrap();
    // sort by name
    let mut d = d.collect::<Vec<_>>();
    d.sort_by(|a, b| a.as_ref().unwrap().path().cmp(&b.as_ref().unwrap().path()));

    for en in d{
        let path = en.unwrap().path();
        println!("opening: {}", path.display());
        v.push(open(path).unwrap().into_rgb8());
    }
    Ok(v)
}

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "0.2", author = "Daniel Alanis")]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    subcmd: SubCommands,
}


#[derive(Subcommand, Debug)]
enum SubCommands{
    Update,
    Run(RunOpts),
}

#[derive(Subcommand, Debug)]
enum Instructions{
    File{ path: PathBuf },
    Args{ args: Vec<String> },
}

#[derive(Parser, Debug)]
struct RunOpts{
        #[clap(short, long)]
        output: Option<PathBuf>,
        #[clap(short, long)]
        deck: Option<PathBuf>,

        #[clap(subcommand)]
        inst: Instructions,
}

fn run_generator(mut opts: RunOpts) -> Result<()>{
    let images = get_images(opts.deck.unwrap())?;

    let inst = match opts.inst{
        Instructions::File{path} => {
            c_instructions(path)?
        },
        Instructions::Args{args} => {
            get_instructions(&args)
        }
    };

    let gen_boards = run(inst)?;
    for (i, b) in gen_boards.iter().enumerate() {
        println!("Board {i:03}: {b:?}");
    }

    let out_path = opts.output.take().unwrap();
    for (index, board) in gen_boards.into_iter().enumerate(){
        let mut path = out_path.clone();
        path.push(format!("image-{index:03}"));
        path.set_extension("png");
        println!("saving to: {}", path.display());
        create_board(board, &images)
            .save(path)?;
    } 

    Ok(())
}

fn main() -> Result<()>{
    let mut opts = Opts::parse();

    if let SubCommands::Run(ref mut opts) = &mut opts.subcmd{
        if opts.output.is_none(){
            opts.output = Some(get_board_path()?);
        }

        if opts.deck.is_none(){
            opts.deck = Some(get_deck_path()?);
        }
    }
    match opts.subcmd{
        SubCommands::Update => {
            install()?;
        },
        SubCommands::Run(opts) => {
            run_generator(opts)?;
        }
    }
    Ok(())
}
