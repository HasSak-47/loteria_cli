use std::{fs::File, io::Read, path::PathBuf};
use crate::error::{LoteriaError, LoteriaResult};
use dirs::{config_dir, desktop_dir, picture_dir};

const VERSION : &str = env!("CARGO_PKG_VERSION");
const FOLDER: &str = "grafica_loteria";

pub fn get_path<F: Fn() -> Option<PathBuf>>(f: F) -> LoteriaResult<String>{
    Ok(f()
        .ok_or(LoteriaError::DirsError)?
        .into_os_string()
        .into_string()
        .unwrap()
    )
}

pub fn get_deck_path() -> LoteriaResult<String>{
    Ok(get_path(picture_dir)? + "/cartas")
}

pub fn get_board_path() -> LoteriaResult<String>{
    Ok(get_path(picture_dir)? + "/baraja")
}
pub fn get_config_path() -> LoteriaResult<String>{
    Ok(get_path(desktop_dir)? + "loteria.txt")
}
