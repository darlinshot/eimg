use std::error::Error;

use clap::{ Parser };

mod eimg;
mod commands;

fn main() -> Result<(), Box<dyn Error>> {
    let eimg = eimg::Eimg::parse();

    let _ = eimg::run(eimg.command);
    Ok(())
}
