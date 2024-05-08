mod cli;
mod utils;
mod updater;

use std::{fs::read_dir, io::Write, path::PathBuf};

#[allow(unused_imports)]
use anyhow::{anyhow, Result};

use cli::{c_instructions, get_instructions, run, ActDebug};
use image::{open, RgbImage};
use loteria_engine::engine::Board;
use utils::get_instruction_path;

use crate::utils::{get_deck_path, get_board_path};

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
#[command(version = "0.3", author = "Daniel Alanis")]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(long)]
    update: bool,

    #[clap(long)]
    debug: bool,

    #[clap(long)]
    override_config: Option<String>,

    #[clap(short, long)]
    output: Option<PathBuf>,

    #[clap(short, long)]
    deck: Option<PathBuf>,

    #[clap(subcommand)]
    inst: Option<Instructions>,
}

#[derive(Subcommand, Debug, Clone)]
enum Instructions{
    File{ path: PathBuf },
    Args{ args: Vec<String> },
}

struct RunOpts{
    override_config: String,
    output         : PathBuf,
    deck           : PathBuf,
    inst           : Vec<Box<dyn ActDebug>>,
}

fn make_run_opts(opts: Opts) -> Result<RunOpts>{
    let inst = match opts.inst{
        Some(inst) => inst,
        None => {
            let path = get_instruction_path(opts.debug)?;
            if !path.exists(){
                return Err(anyhow!("instruction path ({}) does not exist!", path.display()));
            }
            Instructions::File{ path }
        }
    };

    let inst = make_instructions(inst)?;
    let override_config = opts.override_config.unwrap_or("".to_string());
    let output = opts.output.unwrap();
    let deck = opts.deck.unwrap();

    Ok(RunOpts{
        override_config,
        output,
        deck,
        inst,
    })
}

fn make_instructions(inst: Instructions) -> Result<Vec<Box<dyn ActDebug>>>{
    match inst{
        Instructions::File{ path } => {
            let inst = c_instructions(path)?;
            Ok(inst)
        },
        Instructions::Args{ args } => {
            let inst = get_instructions(&args);
            Ok(inst)
        }
    }
}

fn run_generator(opts: RunOpts) -> Result<()>{
    let gen_boards = run(opts.inst)?;
    for (i, b) in gen_boards.iter().enumerate() {
        println!("Board {i:03}: {b:?}");
    }

    let images = get_images(opts.deck)?;
    let out_path = opts.output;
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

    if opts.update{
        return updater::update();
    }

    if opts.output.is_none(){
        opts.output = Some(get_board_path(opts.debug)?);
    }

    if opts.deck.is_none(){
        opts.deck = Some(get_deck_path(opts.debug)?);
    }

    let opts = make_run_opts(opts)?;
    run_generator(opts)?;
    Ok(())
}
