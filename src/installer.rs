use std::{fs::File, io::Read, path::PathBuf};

use dirs::{config_dir, desktop_dir, picture_dir};
use crate::error::{LoteriaError, LoteriaResult};
use crate::utils::*;

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

pub fn install() -> LoteriaResult<()>{
    let _ = std::fs::create_dir(get_deck_path()?);
    let _ = std::fs::create_dir(get_board_path()?);
    let _ = std::fs::File::create(get_config_path()?);


    Ok(())
}
