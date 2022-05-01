use num::Complex;

/// Check if `Complex<f64> c` is in Mandelbrot set using `limit` as max iters.  
///
/// `z.norm_sqr() > 4.0` is a faster way to check if `z` escaped `2r` calculated from the origin of the structure.
///
/// Some(i) = `c` escaped in `i` iterations.
///
/// None = `c` is stable within `i` iterations and thus in the Mandelbrot set.
///
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex::<f64>::default();
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// Map from image space to complex space.
/// `bounds` is the height of the resulting image in pixels, `pixel` is a singular pixel in the image.
/// `Complex` components are points on the complex plane.
pub fn image_plane_to_complex_plane(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_image_plane_to_complex_plane() {
    assert_eq!(
        image_plane_to_complex_plane(
            (100, 200),
            (25, 175),
            Complex { re: -1., im: 1. },
            Complex { re: 1., im: -1. },
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    )
}
