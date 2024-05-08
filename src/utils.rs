use std::path::{PathBuf, Path};
use anyhow::{anyhow, Result};
use dirs::{desktop_dir, picture_dir};

pub static DECK_NAME : &str = "baraja";
pub static BOARD_NAME : &str = "cartas";
pub static INSTUCTIONS_NAME: &str = "loteria";

pub trait Append<T> where Self: Sized
{ fn append(self, v: T) -> Self; }

impl<T> Append<T> for Vec<T >{
    fn append(mut self, v: T) -> Self { self.push(v); self }
}

impl<T> Append<T> for PathBuf where T : AsRef<Path> + Sized{
    fn append(mut self, v: T) -> Self { self.push(v); self }
}

pub fn get_board_path() -> Result<PathBuf>{
    Ok(picture_dir().ok_or(anyhow!("board path not found"))?.append(BOARD_NAME))
}

pub fn get_deck_path() -> Result<PathBuf>{
    Ok(picture_dir().ok_or(anyhow!("deck path not found"))?.append(DECK_NAME))
}

pub fn get_instruction_path() -> Result<PathBuf>{
    let mut p = desktop_dir().ok_or(anyhow!("instructions not found"))?.append(INSTUCTIONS_NAME);
    p.set_extension("txt");
    Ok(p)
}
