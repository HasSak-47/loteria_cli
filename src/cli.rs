use std::{fs::File, io::{BufReader, Read}, path::Path};
use anyhow::*;


pub struct Cli{
    lua_src : String,
}

impl Cli{
    pub fn run(self) -> Result<()>{
        Ok(())
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self>{
        let mut file = BufReader::new( File::open(path)? );
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let lua_src = String::from_utf8(buf)?;

        Ok(Self{ lua_src })
    }
}
