// Ray is a function:
// p(t) = `A + t*`B
// p is 3d position along a line
// A is ray origin
// B is ray direction
// t is a real number, controls position along the line
use crate::types::vec3::Vec3;

pub struct Ray {
    A: Vec3,
    B: Vec3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.A + self.B*t
    }

    pub fn origin(&self) -> &Vec3 {
        &self.A
    }

    pub fn direction(&self) -> &Vec3 {
        &self.B
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { A: origin, B: direction }
    }
}
