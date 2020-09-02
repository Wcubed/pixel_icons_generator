use anyhow::Result;
use image::{GenericImage, GenericImageView, Rgb, RgbImage, SubImage};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const DEFAULT_OUTPUT_DIR: &str = "output";

#[derive(StructOpt)]
#[structopt(
    name = "Pixel Icons Generator",
    version = "1.0",
    about = "Generates a grid of random pixel-art icons. For example, for use as pixel spaceships.",
    author = "Wybe Westra <wybe@ruurdwestra.nl>"
)]
struct Opt {
    /// Where to save the output png.
    /// Defaults to: "output/<random-number>.png" (won't pick the same number twice).
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Width of each icon.
    #[structopt(short = "w", long = "width", default_value = "10")]
    width: u32,

    /// Height of each icon.
    #[structopt(short = "g", long = "height", default_value = "10")]
    height: u32,

    /// How many columns of icons to generate.
    #[structopt(short = "c", long = "columns", default_value = "10")]
    columns: u32,

    /// How many rows of icons to generate.
    #[structopt(short = "r", long = "rows", default_value = "10")]
    rows: u32,

    /// Padding between the icons, and around the border of the image.
    #[structopt(short = "p", long = "padding", default_value = "4")]
    padding: u32,

    /// How many random colors to use.
    #[structopt(short = "k", long = "colors", default_value = "3")]
    colors: usize,

    /// Chance that a pixel will get colored.
    /// Range [0, 100] where 0 is all black, and 100 is all colored.
    #[structopt(short = "n", long = "chance", default_value = "30")]
    color_chance: u32,

    /// Seed for the random generator. Allows for repeatable output.
    /// Random if not specified.
    #[structopt(short = "s", long = "seed")]
    seed: Option<u64>,

    /// Make all icons use the same colors.
    #[structopt(short, long)]
    uniform_colors: bool,

    /// Mirror the icons on the x axis.
    #[structopt(short, long)]
    x_mirror: bool,

    /// Mirror the icons on the y axis.
    #[structopt(short, long)]
    y_mirror: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let out_name = match opt.output {
        Some(path) => {
            match path.extension() {
                Some(ext) if ext == "png" => {
                    // Correct extension.
                    path
                }
                _ => {
                    // Either no extension, or not the correct one.
                    // TODO: proper logging library.
                    println!(
                        "Error: Can only save as png. \"{}\" is not a png.",
                        path.display()
                    );
                    // Exit prematurely.
                    return Ok(());
                }
            }
        }
        None => {
            // No output path specified.
            // Let's generate our own.

            let output_dir = Path::new(DEFAULT_OUTPUT_DIR);
            std::fs::create_dir_all(output_dir)?;
            let mut out = random_not_existing_image_path(output_dir);
            while out.exists() {
                // Whoops, this one already exists.
                // Let's generate another one.
                out = random_not_existing_image_path(output_dir);
            }
            out
        }
    };

    // Check the range on the color chance input.
    if opt.color_chance > 100 {
        println!(
            "Error: \"color_chance\" should be in the range [0, 100]. Got: \"{}\"",
            opt.color_chance
        );
        return Ok(());
    }

    let seed = match opt.seed {
        Some(seed) => seed,
        None => {
            // No seed specified. We create a random one.
            rand::random()
        }
    };

    println!("Seed: {}", seed);

    let mut rng: ChaCha8Rng = rand::SeedableRng::seed_from_u64(seed);

    let img = generate_image(
        &mut rng,
        opt.width,
        opt.height,
        opt.columns,
        opt.rows,
        opt.padding,
        opt.colors,
        opt.color_chance,
        !opt.uniform_colors,
        opt.x_mirror,
        opt.y_mirror,
    );

    println!("Saving to: {}", out_name.display());
    img.save(out_name)?;

    Ok(())
}

/// Get an image filename which does not yet exist.
/// We don't use the seeded rng on purpose, because we want this to be different even if the
/// seed is the same.
fn random_not_existing_image_path(dir: &Path) -> PathBuf {
    dir.join(format!("{}.png", rand::random::<u16>()))
}

/// color_amount: How many random colors will be used.
/// color_chance: [0-100] where 100 is fully colored and 0 is all black.
/// new_colors_for_every_icon: Whether to select new random colors for every icon or not.
fn generate_image(
    rng: &mut ChaCha8Rng,
    icon_width: u32,
    icon_height: u32,
    columns: u32,
    rows: u32,
    padding: u32,
    color_amount: usize,
    color_chance: u32,
    new_colors_for_every_icon: bool,
    mirror_cell_x: bool,
    mirror_cell_y: bool,
) -> RgbImage {
    let mut img = RgbImage::new(
        (icon_width + padding) * columns + padding,
        (icon_height + padding) * rows + padding,
    );

    let mut colors = generate_color_set(rng, color_amount);

    for col in 0..columns {
        for row in 0..rows {
            let x = padding + (icon_width + padding) * col;
            let y = padding + (icon_height + padding) * row;

            if new_colors_for_every_icon {
                colors = generate_color_set(rng, color_amount);
            }

            generate_icon(
                rng,
                &mut img.sub_image(x, y, icon_width, icon_height),
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
/// color_chance: What is the chance a pixel gets a color. Scale [0, 100]
fn generate_icon(
    rng: &mut ChaCha8Rng,
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

fn generate_color_set(rng: &mut ChaCha8Rng, amount: usize) -> Vec<Rgb<u8>> {
    let mut colors = Vec::new();

    for _ in 0..amount {
        colors.push(image::Rgb([rng.gen(), rng.gen(), rng.gen()]));
    }

    colors
}
