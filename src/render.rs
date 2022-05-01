use humansize::{file_size_opts, FileSize};
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder, ImageResult};
use std::fs::File;

use crate::set;
use num::Complex;

pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    // Check that the canvas is of the correct size
    assert!(pixels.len() == bounds.0 * bounds.1);

    for x in 0..bounds.1 {
        for y in 0..bounds.0 {
            let point = set::image_plane_to_complex_plane(bounds, (y, x), upper_left, lower_right);
            pixels[x * bounds.0 + y] = match set::escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
    println!(
        "Render finished for {},{} | {},{}",
        upper_left.re, upper_left.im, lower_right.re, lower_right.im
    );
}

pub fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> ImageResult<()> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    let result = encoder.write_image(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)?;

    // Print image size
    let bytes = std::fs::metadata(filename)?.len();
    println!(
        "Finished, image size: {}",
        bytes.file_size(file_size_opts::CONVENTIONAL).unwrap()
    );

    Ok(())
}
