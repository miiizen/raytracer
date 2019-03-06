use std::ops;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Combine vectors
impl ops::Add for Vec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Scale vector
impl ops::Mul<f64> for Vec {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f64> for Vec {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vec {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Neg for Vec {
    type Output = Self;

    fn neg(self) -> Self {
        Vec {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Index<usize> for Vec {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Out of range"),
        }
    }
}

impl ops::IndexMut<usize> for Vec {
    
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of range"),
        }
    }
}

impl Vec {
    fn new(x: f64, y: f64, z: f64) -> Vec {
        Vec { x, y, z }
    }

    fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    fn squared_length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z)
    }

    fn dot(lhs: Vec, rhs: Vec) -> f64 {
       lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    fn cross(lhs: Vec, rhs: Vec) -> Vec {
        Vec {
            x: lhs.y*rhs.z-lhs.z*rhs.y,
            y: lhs.z*rhs.x-lhs.x*rhs.z,
            z: lhs.x*rhs.y-lhs.y*rhs.x,
        }

    }

    fn unit_vector(&self) -> Self {
        let length = self.length();
        Vec {
            x: self.x / length, 
            y: self.y / length, 
            z: self.z / length, 
        }
    }
}


