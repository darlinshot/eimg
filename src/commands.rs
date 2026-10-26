use std::error::Error;

use crate::eimg::Commands;

pub mod info;

pub fn command_run(command: Commands) -> Result<(), Box<dyn Error>> {
     match command {
         Commands::Info { input } => info::run(input)?,
     }

     Ok(())
}
