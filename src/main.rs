use anyhow::Result;
use image::{GenericImage, GenericImageView, RgbImage, SubImage};
use rand::Rng;
use std::path::Path;

const OUTPUT_DIR: &str = "output";

fn main() -> Result<()> {
    let output_dir = Path::new(OUTPUT_DIR);

    std::fs::create_dir_all(output_dir)?;

    let img = generate_image(100, 100, 3, 1);

    // Get a filename which does not yet exist.
    // TODO: check if it is indeed unique?
    let out_name = output_dir.join(format!("{}.png", rand::random::<u16>()));
    println!("Saving to: {}", out_name.display());
    img.save(out_name)?;

    Ok(())
}

fn generate_image(width: u32, height: u32, n_columns: u32, padding: u32) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    let col_width = (width - ((n_columns + 1) * padding)) / n_columns;
    let col_height = height - padding * 2;

    for i in 0..n_columns {
        let x = padding + (col_width + padding) * i;

        generate_glyph(
            &mut img.sub_image(x, padding, col_width, col_height),
            3,
            10,
            true,
        );
    }

    img
}

///
/// num_colors: How many random colors to use.
/// color_chance: What is the chance a pixel gets a color. Scale [0-100]
fn generate_glyph(
    img: &mut SubImage<&mut RgbImage>,
    num_colors: usize,
    color_chance: u32,
    mirror_x: bool,
) {
    let mut rng = rand::thread_rng();

    let mut colors = Vec::new();

    for _ in 0..num_colors {
        colors.push(image::Rgb([rng.gen(), rng.gen(), rng.gen()]));
    }

    let x_end = if mirror_x {
        // Even or odd width?
        if img.width() % 2 == 0 {
            img.width() / 2
        } else {
            img.width() / 2 + 1
        }
    } else {
        img.width()
    };

    for x in 0..x_end {
        for y in 0..img.height() {
            if rng.gen_range(0, 100) < color_chance {
                let color_id = rng.gen_range(0, colors.len());

                img.put_pixel(x, y, colors[color_id].clone());

                if mirror_x {
                    // Mirror over the x axis.
                    img.put_pixel(img.width() - (x + 1), y, colors[color_id].clone());
                }
            }
        }
    }
}
