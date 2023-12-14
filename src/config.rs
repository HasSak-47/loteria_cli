use std::path::PathBuf;

use dirs::config_dir;

use crate::error::{LoteriaError, LoteriaResult};

fn get_config_path() -> LoteriaResult<PathBuf>{
    let mut path = config_dir()
        .ok_or(LoteriaError::DirsError)?;

    path.push("loteria_config");
    path.set_extension("txt");
    Ok(path)
}
