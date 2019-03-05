use std::ops;
#[derive(Debug, Copy, Clone)]
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

// Dot product
impl ops::Rem for Vec {
    type Output = f64;

    fn rem(self, rhs: Self) -> f64 {
       self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

// Cross product
impl ops::BitXor for Vec {
    type Output = Self;
    
    fn bitxor(self, rhs: Self) -> Self {
        Vec {
            x: self.y*rhs.z-self.z*rhs.y,
            y: self.z*rhs.x-self.x*rhs.z,
            z: self.x*rhs.y-self.y*rhs.x,
        }
    }
}

// Inverse square root
// Used to normalise vectors
impl ops::Not for Vec {
    type Output = Self;

    fn not(self) -> Self {
        let dot: f64 = self % self;
        self * (1.0 / dot.sqrt())
    }
}


