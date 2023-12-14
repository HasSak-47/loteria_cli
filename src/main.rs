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

use crate::{utils::{press_enter_to_continue, get_deck_path}, installer::install};

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
        let path = format!("{}/out.image-{n:03}.png", get_deck_path()?);
        println!("loading: {path}");
        v.push(open(path)?
            .into_rgb8())
    }
    Ok(v)
}

fn handle_error(_error: LoteriaError) {
    use LoteriaError as LT;
    match _error{
        LT::DeckNotFoundAtPath(path) => 
            println!("no ese encontro todas las cartas en: {path}"),
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
    install()?;
    println!("no se generan!!");
    let images = get_images();
    println!("no se generan!!");
    match &images {
        Err(_) => {
            return Err(LoteriaError::DeckNotFoundAtPath(get_deck_path()?))
        },
        _ => {},
    }

    println!("no se generan!!");
    let gen_boards = run()?;
    for b in &gen_boards {
        println!("{b:?}");
    }
    println!("no se generan!!");


    let images = images.unwrap();
    if images.len() != 54{
        return Err(LoteriaError::GenericError("No se encontro todas las imagenes necesarias".to_string()));
    }
    for (index, board) in gen_boards.into_iter().enumerate(){
        let path = format!("{}/out.image-{index:03}.png", get_board_path()?);
        println!("saving {path}");
        create_board(board, &images)
            .save(path)
            .unwrap();
    }


    Ok(())
}
