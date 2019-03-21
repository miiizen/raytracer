use crate::types::vec3::Vec3;
use crate::types::ray::Ray;
use crate::objects::sphere::Sphere;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord { t: 0.0, p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0) }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    spheres: Vec<Sphere>,
}

impl HittableList {
    pub fn new(spheres: Vec<Sphere>) -> Self {
        HittableList { spheres }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far: f64 = t_max;
        for i in &self.spheres {
            if i.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
            }
        }
        hit_anything
    }
}