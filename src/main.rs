mod cli;
mod installer;
mod error;
mod utils;
mod config;

use std::process::exit;

use cli::run;
use error::{LoteriaResult, LoteriaError};
use image::{open, RgbImage};
use loteria_engine::engine::Board;
use utils::get_board_path;

use crate::{utils::{press_enter_to_continue, get_deck_path, Append}, installer::install};

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



fn get_images() -> LoteriaResult<Vec<RgbImage>> {
    let mut v = Vec::new();
    for n in 0..54 {
        let mut path = get_deck_path()?.append(format!("image-{n:03}"));
        path.set_extension("png");
        println!("loading: {}", path.display());
        v.push(open(path)?
            .into_rgb8())
    }
    Ok(v)
}

fn handle_error(_error: LoteriaError) {
    use LoteriaError as LT;
    match _error{
        LT::DeckNotFoundAtPath(path) => 
            println!("didn't found cards at: {path}"),
        LT::GenericError(error) => 
            println!("error: {error}!"),
        _ => {},
    }
    press_enter_to_continue();
    exit(-1);
}

fn main(){
    match panicked_main(){
        Ok(_) => {},
        Err(err) => handle_error(err),
    }
}

fn panicked_main() -> LoteriaResult<()>{
    // instala
    println!("making sure stuff is installed");
    install()?;
    println!("getting images...");
    let images = get_images();
    match &images {
        Err(_) => {
            return Err(LoteriaError::DeckNotFoundAtPath(
                get_deck_path()?
                    .to_str()
                    .unwrap()
                    .to_string()
            ))
        },
        _ => {},
    }

    println!("generating boards");
    let gen_boards = run()?;
    for b in &gen_boards {
        println!("{b:?}");
    }

    let images = images.unwrap();
    if images.len() != 54{
        return Err(LoteriaError::GenericError("Not enough cards found!".to_string()));
    }
    for (index, board) in gen_boards.into_iter().enumerate(){
        let mut path = get_board_path()?.append(format!("out.image-{index:03}"));
        path.set_extension(".png");
        println!("saving to: {}", path.display());
        create_board(board, &images)
            .save(path)
            .unwrap();
    }


    Ok(())
}
