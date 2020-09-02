use anyhow::Result;
use image::{GenericImage, GenericImageView, RgbImage, SubImage};
use rand::Rng;
use std::path::Path;

const OUTPUT_DIR: &str = "output";

fn main() -> Result<()> {
    let output_dir = Path::new(OUTPUT_DIR);

    std::fs::create_dir_all(output_dir)?;

    let img = generate_image(10, 20, 5, 4, 4, 3, 30);

    // Get a filename which does not yet exist.
    // TODO: check if it is indeed unique?
    let out_name = output_dir.join(format!("{}.png", rand::random::<u16>()));
    println!("Saving to: {}", out_name.display());
    img.save(out_name)?;

    Ok(())
}

/// color_chance: [0-100] where 100 is fully colored.
fn generate_image(
    cell_width: u32,
    cell_height: u32,
    columns: u32,
    rows: u32,
    padding: u32,
    colors_per_cell: usize,
    color_chance: u32,
) -> RgbImage {
    let mut img = RgbImage::new(
        (cell_width + padding) * columns + padding,
        (cell_height + padding) * rows + padding,
    );

    for col in 0..columns {
        for row in 0..rows {
            let x = padding + (cell_width + padding) * col;
            let y = padding + (cell_height + padding) * row;

            generate_glyph(
                &mut img.sub_image(x, y, cell_width, cell_height),
                colors_per_cell,
                color_chance,
                true,
            );
        }
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
