use dirs::{desktop_dir, picture_dir};

use crate::error::{LoteriaError, LoteriaResult};
use crate::utils::{*};

use std::fs::{self, DirEntry};
use std::io::Write;

const VERSION : &str = env!("CARGO_PKG_VERSION");
const FOLDER: &str = "grafica_loteria";

fn get_entries(path: String) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .map(|x| x
             .unwrap()
             .path()
             .into_os_string()
             .into_string()
             .unwrap())
        .collect()
}

fn find_in(entries: &Vec<String>, entry: &String) -> bool{
    entries
        .iter()
        .find(|x| *x == entry )
        .is_some()
}

pub fn install() -> LoteriaResult<()>{
    let mut desktop_entries = get_entries(get_path(desktop_dir)?);
    let mut picture_entries = get_entries(get_path(picture_dir)?);
    desktop_entries.sort();
    picture_entries.sort();

    let deck = get_deck_path()?;
    let board = get_board_path()?;
    let instruction = get_instruction_path()?;

    if !find_in(&desktop_entries, &instruction){
        println!("created instruction: {}", instruction);
        let mut file = fs::File::create(instruction).unwrap();
        let _ = file.write_all(DEFAULT.as_bytes());
    }
    if !find_in(&picture_entries, &board){
        println!("created board, {}", board);
        let _ = fs::create_dir(board).unwrap();
    }
    if !find_in(&picture_entries, &deck){
        println!("created deck {}", deck);
        let _ = fs::create_dir(deck).unwrap();
    }

    Ok(())
}
