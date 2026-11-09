use std::error::Error;
use std::path::PathBuf;

use image;
use image_hasher::{ HasherConfig, HashAlg };

pub fn run(input1: PathBuf, input2: PathBuf, hash_size: u32) -> Result<(), Box<dyn Error>> {
    let img1 = image::open(input1)?.to_rgb8();
    let img2 = image::open(input2)?.to_rgb8();

    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::DoubleGradient)
        .hash_size(hash_size, hash_size)
        .to_hasher();

    let hash1 = hasher.hash_image(&img1);
    let hash2 = hasher.hash_image(&img2);

    let distance = hash1.dist(&hash2);

    println!("{}", distance);

    Ok(())
    
}
