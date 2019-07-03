#![allow(dead_code)]
#![allow(unused_imports)]

mod vec3;

use std::fs;
use std::io;
use std::io::prelude::*;

use vec3::Vec3;

// image size
const COLUMNS: u32 = 200;
const ROWS: u32 = 100;

fn main() -> io::Result<()> {
    let mut file = fs::File::create("image.ppm")?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", COLUMNS, ROWS)?;
    writeln!(file, "255")?;

    for row in (0..ROWS).rev() {
        for col in 0..COLUMNS {
            let rgb = Vec3::new(
                col as f32 / COLUMNS as f32,
                row as f32 / ROWS as f32,
                0.2_f32,
            );

            let ir = (255.99 * rgb.r()) as u32;
            let ig = (255.99 * rgb.g()) as u32;
            let ib = (255.99 * rgb.b()) as u32;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
