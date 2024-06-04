use std::{io::stdin, path::{Path, PathBuf}};
use anyhow::{anyhow, Result};
use dirs::{desktop_dir, picture_dir};

pub static DECK_NAME : &str = "baraja";
pub static BOARD_NAME : &str = "cartas";
pub static INSTUCTIONS_NAME: &str = "loteria.txt";

pub trait Append<T> where Self: Sized
{ fn append(self, v: T) -> Self; }

impl<T> Append<T> for Vec<T >{
    fn append(mut self, v: T) -> Self { self.push(v); self }
}

impl<T> Append<T> for PathBuf where T : AsRef<Path> + Sized{
    fn append(mut self, v: T) -> Self { self.push(v); self }
}

pub fn get_board_path(debug: bool) -> Result<PathBuf>{
    let path = picture_dir()
        .ok_or(anyhow!("Pictures not found!"))?
        .append(BOARD_NAME);
    if debug{
        println!("using default board path {}", path.display());
    }
    Ok(path)
}

pub fn get_deck_path(debug: bool) -> Result<PathBuf>{
    let path = picture_dir()
        .ok_or(anyhow!("Pictures not found!"))?
        .append(DECK_NAME);
    if debug{
        println!("using default deck path {}", path.display());
    }
    Ok(path)
}

pub fn get_instruction_path(debug: bool) -> Result<PathBuf>{
    let path = desktop_dir()
        .ok_or(anyhow!("Desktop not found!"))?
        .append(INSTUCTIONS_NAME);
    if debug{
        println!("using default instruction path {}", path.display());
    }
    Ok(path)
}

pub fn wait_enter(){
    stdin().read_line(&mut String::new()).unwrap();
}
