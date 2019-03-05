mod vector;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let nx: i32 = 200;
    let ny: i32 = 100;

    let head = format!("P3\n{} {}\n255\n", nx.to_string(), ny.to_string());
    io::stdout().write(head.as_bytes())?;
    for y in (0..ny).rev() {
        for x in 0..nx {
            let r: f64 = x as f64 / nx as f64;
            let g: f64 = y as f64 / ny as f64;
            let b: f64 = 0.2;
            let ir: i32 = (255.9 * r) as i32;
            let ig: i32 = (255.9 * g) as i32;
            let ib: i32 = (255.9 * b) as i32;
            let rgb = format!("{} {} {}\n", ir, ig, ib);
            io::stdout().write(rgb.as_bytes())?;
        }
    }
    Ok(())
}

