use anyhow::Result;
use image::{GenericImage, GenericImageView, Rgb, RgbImage, SubImage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::path::Path;

const OUTPUT_DIR: &str = "output";

fn main() -> Result<()> {
    let output_dir = Path::new(OUTPUT_DIR);

    std::fs::create_dir_all(output_dir)?;

    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(1);

    let img = generate_image(&mut rng, 10, 20, 5, 4, 4, 3, 30, false, true, false);

    // Get a filename which does not yet exist.
    // We don't use the seeded rng on purpose, because we want this to be different even if the
    // seed is the same.
    // TODO: check if it is indeed unique?
    let out_name = output_dir.join(format!("{}.png", rand::random::<u16>()));
    println!("Saving to: {}", out_name.display());
    img.save(out_name)?;

    Ok(())
}

/// color_amount: How many random colors will be used.
/// color_chance: [0-100] where 100 is fully colored and 0 is all black.
/// new_colors_for_every_cell: Whether to select new random colors for every cell or not.
fn generate_image(
    rng: &mut StdRng,
    cell_width: u32,
    cell_height: u32,
    columns: u32,
    rows: u32,
    padding: u32,
    color_amount: usize,
    color_chance: u32,
    new_colors_for_every_cell: bool,
    mirror_cell_x: bool,
    mirror_cell_y: bool,
) -> RgbImage {
    let mut img = RgbImage::new(
        (cell_width + padding) * columns + padding,
        (cell_height + padding) * rows + padding,
    );

    let mut colors = generate_color_set(rng, color_amount);

    for col in 0..columns {
        for row in 0..rows {
            let x = padding + (cell_width + padding) * col;
            let y = padding + (cell_height + padding) * row;

            if new_colors_for_every_cell {
                colors = generate_color_set(rng, color_amount);
            }

            generate_glyph(
                rng,
                &mut img.sub_image(x, y, cell_width, cell_height),
                &colors,
                color_chance,
                mirror_cell_x,
                mirror_cell_y,
            );
        }
    }

    img
}

///
/// color_chance: What is the chance a pixel gets a color. Scale [0-100]
fn generate_glyph(
    rng: &mut StdRng,
    img: &mut SubImage<&mut RgbImage>,
    colors: &Vec<Rgb<u8>>,
    color_chance: u32,
    mirror_x: bool,
    mirror_y: bool,
) {
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

    let y_end = if mirror_y {
        // Even or odd height?
        if img.height() % 2 == 0 {
            img.height() / 2
        } else {
            img.height() / 2 + 1
        }
    } else {
        img.height()
    };

    for x in 0..x_end {
        for y in 0..y_end {
            if rng.gen_range(0, 100) < color_chance {
                let color_id = rng.gen_range(0, colors.len());

                img.put_pixel(x, y, colors[color_id].clone());

                if mirror_x {
                    // Mirror over the x axis.
                    img.put_pixel(img.width() - (x + 1), y, colors[color_id].clone());
                }
                if mirror_y {
                    // Mirror over the y axis.
                    img.put_pixel(x, img.height() - (y + 1), colors[color_id].clone());
                }
                if mirror_x && mirror_y {
                    // Mirror across the center.
                    img.put_pixel(
                        img.width() - (x + 1),
                        img.height() - (y + 1),
                        colors[color_id].clone(),
                    );
                }
            }
        }
    }
}

fn generate_color_set(rng: &mut StdRng, amount: usize) -> Vec<Rgb<u8>> {
    let mut colors = Vec::new();

    for _ in 0..amount {
        colors.push(image::Rgb([rng.gen(), rng.gen(), rng.gen()]));
    }

    colors
}
