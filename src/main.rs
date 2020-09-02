use anyhow::Result;
use image::{ImageBuffer, RgbImage};
use rand::Rng;
use std::path::Path;

const OUTPUT_DIR: &str = "output";

fn main() -> Result<()> {
    let output_dir = Path::new(OUTPUT_DIR);

    std::fs::create_dir_all(output_dir)?;

    let img = generate_image(3, 10);

    // Get a filename which does not yet exist.
    // TODO: check if it is indeed unique?
    let out_name = output_dir.join(format!("{}.png", rand::random::<u16>()));
    println!("Saving to: {}", out_name.display());
    img.save(out_name)?;

    Ok(())
}

///
/// num_colors: How many random colors to use.
/// color_chance: What is the chance a pixel gets a color. Scale [0-100]
fn generate_image(num_colors: usize, color_chance: u32) -> RgbImage {
    let mut rng = rand::thread_rng();

    let mut img: RgbImage = ImageBuffer::new(256, 256);
    let mut colors = Vec::new();

    for _ in 0..num_colors {
        colors.push(image::Rgb([rng.gen(), rng.gen(), rng.gen()]));
    }

    for pixel in img.pixels_mut() {
        if rng.gen_range(0, 100) < color_chance {
            let color_id = rng.gen_range(0, colors.len());
            *pixel = colors[color_id];
        }
    }

    img
}
