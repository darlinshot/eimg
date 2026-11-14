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
    Similarity {
        input1: PathBuf,
        input2: PathBuf,
        hash_size: u32,
    },
    BulkSimilarity {
        path: PathBuf,
        hash_size: u32,
        min_score: u32
    }
}

pub fn run(command: Commands) -> Result<(), Box<dyn Error>> {
    commands::run_command(command)?;

    Ok(())
}
