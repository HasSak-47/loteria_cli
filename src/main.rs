mod cli;
mod installer;
mod error;
mod utils;

use error::LoteriaResult;

fn main() -> LoteriaResult<()>{
    let _ = installer::install();
    Ok(())
}
