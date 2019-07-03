use std::fs;
use std::io;
use std::io::prelude::*;

// image size
const COLUMNS: u32 = 200;
const ROWS: u32 = 100;

fn main() -> io::Result<()> {
    let mut file = fs::File::create("my2.ppm")?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", COLUMNS, ROWS)?;
    writeln!(file, "255")?;    

    for row in (0..ROWS).rev() {
        for col in 0..COLUMNS {
            let r = col as f32 / COLUMNS as f32; // red
            let g = row as f32 / ROWS as f32; // green
            let b = 0.2_f32; // blue

            let ir = (255.99 * r) as u32;
            let ig = (255.99 * g) as u32;
            let ib = (255.99 * b) as u32;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
