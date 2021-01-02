use std::io::{Stdout, Write};

use crate::util::clamp;
use crate::vector3::Vector3 as Color;

pub(crate) fn write_color(mut stdout: Stdout, pixel_color: Color, samples_per_pixel: u8) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    stdout.write_fmt(format_args!("{} {} {}\n",
                                  (256.0 * clamp(r, 0.0, 0.999)) as u8,
                                  (256.0 * clamp(g, 0.0, 0.999)) as u8,
                                  (256.0 * clamp(b, 0.0, 0.999)) as u8)).unwrap()
}