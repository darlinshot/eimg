use std::error::Error;

use crate::eimg::Commands;

pub mod info;
pub mod similarity;
pub mod bulk_similarity;

pub fn run_command(command: Commands) -> Result<(), Box<dyn Error>> {
    match command {
        Commands::Info { input } => info::run(input)?,
        Commands::Similarity { input1, input2, hash_size } => similarity::run(input1, input2, hash_size)?,
        Commands::BulkSimilarity { path, hash_size, min_score } => bulk_similarity::run(path, hash_size, min_score)?,
    }

     Ok(())
}
