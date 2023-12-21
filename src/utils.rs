use std::{fs::File, io::Read, path::{PathBuf, Path}};
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

pub static BOARD_NAME : &str = "baraja";
pub static DECK_NAME : &str = "cartas";
pub static INSTUCTIONS_NAME: &str = "loteria";

pub trait Append<T> where Self: Sized
{ fn append(self, v: T) -> Self; }

impl<T> Append<T> for Vec<T >{
    fn append(mut self, v: T) -> Self { self.push(v); self }
}

impl<T> Append<T> for PathBuf where T : AsRef<Path> + Sized{
    fn append(mut self, v: T) -> Self { self.push(v); self }
}

pub fn get_board_path() -> LoteriaResult<PathBuf>{
    Ok(picture_dir().ok_or(LoteriaError::DirsError)?.append(BOARD_NAME))
}

pub fn get_deck_path() -> LoteriaResult<PathBuf>{
    Ok(picture_dir().ok_or(LoteriaError::DirsError)?.append(DECK_NAME))
}

pub fn get_instruction_path() -> LoteriaResult<PathBuf>{
    let mut p = desktop_dir().ok_or(LoteriaError::DirsError)?.append(INSTUCTIONS_NAME);
    p.set_extension("txt");
    Ok(p)
}
pub const DEFAULT: &str =
"SetCount 8
RandomCenterMarkPair
";

pub fn press_enter_to_continue() {
    let mut s = String::new();
    println!("presione enter para continuar");
    let _ = std::io::stdin().read_line(&mut s);
}
