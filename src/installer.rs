use dirs::{desktop_dir, picture_dir};
use anyhow::{Result, anyhow};

use crate::utils::{*};

use std::fs;
use std::io::Write;
use std::path::PathBuf;

const VERSION : &str = env!("CARGO_PKG_VERSION");
const FOLDER: &str = "grafica_loteria";

fn get_entries(path: PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .map(|x| x
            .unwrap()
            .path()
        )
        .collect()
}

fn find_in(entries: &Vec<PathBuf>, entry: &PathBuf) -> bool{
    entries
        .iter()
        .find(|x| *x == entry )
        .is_some()
}

pub fn install() -> Result<()>{
    let mut desktop_entries = get_entries(desktop_dir().ok_or(anyhow!("desktop not found"))?);
    let mut picture_entries = get_entries(picture_dir().ok_or(anyhow!("pictures not found"))?);
    desktop_entries.sort();
    picture_entries.sort();

    let deck = get_deck_path()?;
    let board = get_board_path()?;
    let instruction = get_instruction_path()?;

    if !find_in(&desktop_entries, &instruction){
        println!("created instruction file at: {}", instruction.display());
        let mut file = fs::File::create(instruction).unwrap();
        let _ = file.write_all(DEFAULT.as_bytes());
    }
    if !find_in(&picture_entries, &board){
        println!("created board folder at: {}", board.display());
        let _ = fs::create_dir(board).unwrap();
    }
    if !find_in(&picture_entries, &deck){
        println!("created deck folder at: {}", deck.display());
        let _ = fs::create_dir(deck).unwrap();
    }

    Ok(())
}
