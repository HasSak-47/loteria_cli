use std::{fs::File, io::{BufReader, Read}, path::Path};
use anyhow::*;
use image::{GenericImageView, RgbaImage};
use loteria_engine::{init, run, BOARD};

use crate::{debug, Opts};


pub struct Cli{
    lua_src : String,

    opts: Opts,
}

impl Cli{
    pub fn run(self) -> Result<()>{
        init(self.lua_src.as_str())?;
        let tables = run()?;
        let (count, total, (board_w, board_h)) = unsafe{(
            BOARD.get_count(),
            BOARD.get_total(),
            BOARD.get_dims(),
        )};
        debug!(self.opts, "count: {count}");
        debug!(self.opts, "total: {total}");
        debug!(self.opts, "size : {board_w}, {board_h}");

        let mut images = Vec::new();
        for i in 0..count{
            let mut path = self.opts.deck.clone();
            path.push(format!("{}{i:03}", self.opts.in_format));
            path.set_extension("png");
            debug!(self.opts, "loading image {}", path.display());

            let image = image::open(path)?;
            images.push(image);

        }

        let (width, height) = images[0].dimensions();
        let mut buffer = RgbaImage::new(width * board_w as u32, height * board_h as u32);

        for indx in 0..total{
            debug!(self.opts, "generating board {indx}..");
            let table = &tables[indx];
            for ij in 0..board_w * board_h{
                let i = ij % board_w;
                let j = ij / board_w;

                debug!(self.opts, "getting ({i}, {j})");
                let card_index = table.get(i, j).unpack();
                for w in 0..width{
                    for h in 0..height{
                        let card_p = images[card_index].get_pixel(w as u32, h as u32);
                        let buff_p = buffer.get_pixel_mut(i as u32 * width + w , j as u32 * height + h);
                        *buff_p = card_p;
                    }
                }
            }

            let mut path = self.opts.output.clone();
            path.push(format!("{}{indx:03}", self.opts.out_format));
            path.set_extension("png");

            debug!(self.opts, "saving at {}..", path.display());
            buffer.save(path)?;
        }

        Ok(())
    }

    pub fn config<P: AsRef<Path>>(opts: Opts, path: P) -> Result<Self>{
        let path = path.as_ref();
        debug!(opts, "lua path: {}", path.display());
        let mut file = BufReader::new( File::open(path)? );
        let mut buf = Vec::new();
        

        file.read_to_end(&mut buf)?;
        let lua_src = String::from_utf8(buf)?;
        debug!(opts, "lua src:\n{lua_src}");

        Ok(Self{
            opts,
            lua_src,
        })
    }
}
