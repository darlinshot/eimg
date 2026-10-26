use std::error::Error;
use clap::{ Parser, Subcommand };

use std::path::PathBuf;

use crate::commands;

#[derive(Parser, Debug)]
#[command(name="eimg", version, about="Ei image tools.")]
pub struct Eimg {
    #[command(subcommand)]
    pub command: Commands 
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Info {
        input: PathBuf,
    },
}

pub fn run(command: Commands) -> Result<(), Box<dyn Error>> {
    let _ = commands::command_run(command);

    Ok(())
}
