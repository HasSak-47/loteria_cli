use std::path::PathBuf;
use anyhow::{Result, anyhow};

use dirs::config_dir;

fn get_config_path() -> Result<PathBuf>{
    let mut path = config_dir()
        .ok_or(anyhow!("config dir not found"))?;

    path.push("loteria_config");
    path.set_extension("txt");

    Ok(path)
}
