pub mod types;
pub mod objects;
use types::vec3::Vec3;
use types::ray::Ray;
use objects::camera::Camera;
use objects::sphere::Sphere;
use objects::hittable::{Hittable, HittableList, HitRecord};
use std::io::{self, Write};

use rand::Rng;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3 = Vec3::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))*2.0 - Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = Vec3::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))*2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

// linear interpolation of blue to white
// blended_value = (1-t)*start_value + t*end_value, t: 0 -> 1
fn colour(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(r, 0.001, std::f64::MAX, &mut rec) {
        let target: Vec3 = rec.p + rec.normal + random_in_unit_sphere();
        return colour(&Ray::new(rec.p, target - rec.p), world)*0.5;
    }
    else {
        let unit_direction = r.direction().unit_vector();
        let t: f64 = 0.5 *(unit_direction[1] + 1.0);
        Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
    }
}

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let nx: i32 = 200;
    let ny: i32 = 100;
    // Number of samples
    let ns: i32 = 100;

    let head = format!("P3\n{} {}\n255\n", nx.to_string(), ny.to_string());
    io::stdout().write(head.as_bytes())?;

    let lower_left = Vec3::new(-2.0, -2.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world = HittableList::new(vec![Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5 ), Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)]);

    let cam: Camera = Camera::new();

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f64 = (x as f64 + rng.gen_range(0.0, 1.0)) / nx as f64;
                let v: f64 = (y as f64 + rng.gen_range(0.0, 1.0)) / ny as f64;

                let r: Ray = cam.get_ray(u, v);

                let p: Vec3  = r.point_at_parameter(2.0);
                col += colour(&r, &world);
            }
            // Colours
            col /= ns as f64;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let ir: i32 = (255.9 * col[0]) as i32;
            let ig: i32 = (255.9 * col[1]) as i32;
            let ib: i32 = (255.9 * col[2]) as i32;
            let rgb = format!("{} {} {}\n", ir, ig, ib);
            io::stdout().write(rgb.as_bytes())?;
        }
    }
    Ok(())
}
