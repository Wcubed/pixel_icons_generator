use anyhow::Result;
use image::{ImageBuffer, RgbImage};
use std::path::Path;

const OUTPUT_DIR: &str = "output";

fn main() -> Result<()> {
    let output_dir = Path::new(OUTPUT_DIR);

    std::fs::create_dir_all(output_dir)?;

    let img = generate_image();

    // Get a filename which does not yet exist.
    // TODO: check if it is indeed unique?
    let out_name = output_dir.join(format!("{}.png", rand::random::<u16>()));
    println!("Saving to: {}", out_name.display());
    img.save(out_name)?;

    Ok(())
}

fn generate_image() -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(256, 256);

    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([rand::random(), rand::random(), rand::random()]);
    }

    img
}
