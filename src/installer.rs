use crate::error::{LoteriaError, LoteriaResult};
use crate::utils::*;

const VERSION : &str = env!("CARGO_PKG_VERSION");
const FOLDER: &str = "grafica_loteria";

pub fn install() -> LoteriaResult<()>{
    let _ = std::fs::create_dir(get_deck_path()?);
    let _ = std::fs::create_dir(get_board_path()?);
    let _ = std::fs::File::create(get_config_path()?);


    Ok(())
}
