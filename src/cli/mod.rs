use std::env::args;

use crate::error::LoteriaResult;

fn exe() -> LoteriaResult<()>{
    Ok(())
}

pub fn run() -> LoteriaResult<()> {
    let args : Vec<_> = args().collect();
    if args.len() == 1{

    }

    Ok(())
}
