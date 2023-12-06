
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
    get_path(picture_dir)? + "/cartas"
}

pub fn get_pict_path() -> String{
    get_path(picture_dir)? + "/baraja"
}
pub fn get_conf_path() -> String{
    get_path(desktop_dir)? + "loteria.txt"
}
