use vec3;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let nx: i32 = 200;
    let ny: i32 = 100;

    let head = format!("P3\n{} {}\n255\n", nx.to_string(), ny.to_string());
    io::stdout().write(head.as_bytes())?;
    for y in (0..ny).rev() {
        for x in 0..nx {
            let col = Vec3::new(x as f64 / nx as f64, 
                               y as f64 / ny as f64,
                               0.2);
            let ir: i32 = (255.9 * col[0]) as i32;
            let ig: i32 = (255.9 * col[1]) as i32;
            let ib: i32 = (255.9 * col[2]) as i32;
            let rgb = format!("{} {} {}\n", ir, ig, ib);
            io::stdout().write(rgb.as_bytes())?;
        }
    }
    Ok(())
}

