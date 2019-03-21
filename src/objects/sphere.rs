use crate::types::vec3::Vec3;
use crate::types::ray::Ray;
use crate::objects::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {

    // Test intersection with sphere by checking discriminant of quadratic eqn
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = *r.origin() - self.center;
        let a: f64 = Vec3::dot(*r.direction(), *r.direction());
        let b: f64 = 2.0 * Vec3::dot(oc, *r.direction());
        let c: f64 = Vec3::dot(oc, oc) - self.radius*self.radius;
        let discriminant: f64 = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let temp: f64 = (-b - (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            let temp = (-b + (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}