use std::error::Error;
use std::path::PathBuf;

pub fn run(input:PathBuf) -> Result<(), Box<dyn Error>> {
    let img = image::open(&input)?;

    println!("Image: {}", input.display());
    println!("Width: {}", img.width());
    println!("Height: {}", img.height());

    Ok(())
}
