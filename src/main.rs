use std::error::Error;
use std::path::PathBuf;

use clap:: { Parser, Subcommand };

#[derive(Parser)]
#[command(name="eimg")]
#[command(version, about="Ei image tools.")]
struct Eimg {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Subcommand)]
enum Commands {
    Info{input: PathBuf,},
    Ratio{input: PathBuf,},
}
fn main() -> Result<(), Box<dyn Error>> {
    let eimg = Eimg::parse();

    match eimg.command {
        Commands::Info { input } => {
            let img = image::open(&input)?;
            println!("Image: {}", input.display());
        }

        Commands::Ratio { input } => {
            println!("Nothing here yet");
        }
    }

    Ok(())
}
