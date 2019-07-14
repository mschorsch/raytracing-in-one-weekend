#![allow(dead_code)]
#![allow(unused_imports)]

mod ray;
mod vec3;
use std::fs;
use std::io;
use std::io::prelude::*;

use ray::Ray;
use vec3::{unit_vector, Vec3};

// image size
const COLUMNS: u32 = 200;
const ROWS: u32 = 100;

fn main() -> io::Result<()> {
    let mut file = fs::File::create("image.ppm")?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", COLUMNS, ROWS)?;
    writeln!(file, "255")?;

    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    for j in (0..ROWS).rev() {
        for i in 0..COLUMNS {
            let u = i as f32 / COLUMNS as f32;
            let v = j as f32 / ROWS as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);

            let ir = (255.99 * col.0) as u32;
            let ig = (255.99 * col.1) as u32;
            let ib = (255.99 * col.2) as u32;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}

fn color(r: &Ray) -> Vec3 {
    // linear blend; lineai interpolation; lerp
    // blended_value = (1-t)*start_value + t*end_value
    let unit_direction = unit_vector(r.direction); // unit vector -1 <= x <= 1
    let t = 0.5 * (unit_direction.y() + 1.0); // scale to 0 <= x <= 1
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) /* white */ + t * Vec3(0.5, 0.7, 1.0) /* blue */ // blend blue to white
}
