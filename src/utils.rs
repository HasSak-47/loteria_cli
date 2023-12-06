use std::{fs::File, io::Read, path::PathBuf};
use crate::error::{LoteriaError, LoteriaResult};
use dirs::{config_dir, desktop_dir, picture_dir};

pub fn get_path<F: Fn() -> Option<PathBuf>>(f: F) -> LoteriaResult<String>{
    Ok(f()
        .ok_or(LoteriaError::DirsError)?
        .into_os_string()
        .into_string()
        .unwrap()
    )
}

#[cfg(target_family= "unix")]
mod os_th{
pub static BOARD_PATH : &str = "/baraja";
pub static DECK_PATH : &str = "/cartas";
pub static INSTUCTIONS_PATH : &str = "/loteria.txt";
}

#[cfg(target_windows = "windows")]
mod os_th{
static BOARD_PATH : &str = "\\baraja";
static DECK_PATH : &str = "\\cartas";
static INSTUCTIONS_PATH : &str = "\\loteria.txt";
}

use self::os_th::*;

pub fn get_deck_path() -> LoteriaResult<String>{
    Ok(get_path(picture_dir)? + DECK_PATH)
}

pub fn get_board_path() -> LoteriaResult<String>{
    Ok(get_path(picture_dir)? + BOARD_PATH)
}

pub fn get_instruction_path() -> LoteriaResult<String>{
    Ok(get_path(desktop_dir)? + INSTUCTIONS_PATH)
}
pub const DEFAULT: &str =
"SetCount 8
RandomCenterMarkPair
";
