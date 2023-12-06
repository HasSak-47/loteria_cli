mod cli;
mod installer;
mod error;
mod utils;

use cli::run;
use error::LoteriaResult;

fn main() -> LoteriaResult<()>{
    let _ = installer::install();
    let boards = run()?;
    for b in boards {
        println!("{b:?}");
    }


    Ok(())
}
