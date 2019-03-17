// Ray is a function:
// p(t) = `A + t*`B
// p is 3d position along a line
// A is ray origin
// B is ray direction
// t is a real number, controls position along the line
mod vector;

struct Ray {
    origin: Vec,
    direction: Vec,
}

impl Ray {
    point_at_parameter(t: f64) -> Vec {
        origin + t*direction
    }
}
