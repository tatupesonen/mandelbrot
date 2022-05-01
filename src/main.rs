#![allow(dead_code)]
use clap::Parser;
mod parser;
mod render;
mod set;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Resolution
    #[clap(short, long, default_value_t = 36000)]
    width: usize,

    /// Coordinates
    #[clap(short, long, allow_hyphen_values(true), default_value = "-1.2,0.35")]
    top_left: String,

    #[clap(short, long, allow_hyphen_values(true), default_value = "-1.0,0.2")]
    bottom_right: String,

    #[clap(short, long, default_value = "fractal.png")]
    filename: String,
}

fn main() {
    let threads = num_cpus::get();
    println!("Using {} threads...", threads);

    let args = Args::parse();
    let width: usize = args.width;
    let bounds = (width, (width as f64 / (4.0 / 3.0)) as usize);
    let top_left = parser::parse_complex(&args.top_left).unwrap();
    let bottom_right = parser::parse_complex(&args.bottom_right).unwrap();

    let rows_per_band = bounds.1 / threads + 1;

    let mut pixels = vec![0; bounds.0 * bounds.1];
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left =
                set::image_plane_to_complex_plane(bounds, (0, top), top_left, bottom_right);
            let band_lower_right = set::image_plane_to_complex_plane(
                bounds,
                (bounds.0, top + height),
                top_left,
                bottom_right,
            );

            spawner.spawn(move |_| {
                render::render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    })
    .unwrap();
    render::write_image(&args.filename, &pixels, bounds).expect("Failed writing image.");
}
