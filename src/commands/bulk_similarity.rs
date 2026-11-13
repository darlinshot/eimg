use std::error::Error;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

use image_hasher::{ ImageHash, HasherConfig, HashAlg} ;

struct ImageRecord {
    path: PathBuf,
    hash: ImageHash
}

pub fn run(path: PathBuf, hash_size: u32, min_score: u32) -> Result<(), Box<dyn Error>> {
    let mut images: Vec<ImageRecord> = Vec::new();

    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::DoubleGradient)
        .hash_size(hash_size, hash_size)
        .to_hasher();

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        let is_image = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                matches!(
                    ext.to_lowercase().as_str(),
                    "jpg" | "jpeg" | "png" | "bmp" | "gif" | "webp" | "tiff"
                )
            })
            .unwrap_or(false);
        if !is_image {
            continue;
        }

        let img = image::open(&file_path)?;
        println!("{:?}", file_path);
        
        let hash = hasher.hash_image(&img);

        images.push(ImageRecord {
            path: file_path,
            hash: hash
        })
    }

    println!("Processed {} image(s).", images.len());

    for i in 0..images.len() {
        for j in (i+1)..images.len() {
            let dist = images[i].hash.dist(&images[j].hash);

            if dist <= min_score {
                println!(
                    "{:?} <-> {:?} (Distance: {})",
                    images[i].path,
                    images[j].path,
                    dist
                )
            }

        }
    }

    Ok(())
}