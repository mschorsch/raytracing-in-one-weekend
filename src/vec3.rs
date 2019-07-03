use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn r(&self) -> f32 {
        self.x
    }

    pub fn g(&self) -> f32 {
        self.y
    }

    pub fn b(&self) -> f32 {
        self.z
    }

    pub fn rgb(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        // scalar product
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        // cross product
        let x = self.y * other.z - self.z * other.y;
        let y = -(self.x * other.z - self.z * other.x);
        let z = self.x * other.y - other.y * self.x;
        Vec3::new(x, y, z)
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1_f32 / self.length();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

//
// operator overloading
//

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self {
        self + &other
    }
}

impl<'a> ops::Add<&'a Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: &'a Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        self - &other
    }
}

impl<'a> ops::Sub<&'a Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: &'a Vec3) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        self * &other
    }
}

impl<'a> ops::Mul<&'a Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: &'a Vec3) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl<'a> ops::Mul<&'a Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> Self {
        self / &other
    }
}

impl<'a> ops::Div<&'a Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: &'a Vec3) -> Self {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl<'a> ops::AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &'a Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<'a> ops::SubAssign<&'a Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &'a Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<'a> ops::MulAssign<&'a Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &'a Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl<'a> ops::DivAssign<&'a Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: &'a Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        let k = 1_f32 / t;
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_make_unit_vector() {
        //        let mut v = Vec3::new(2.0, 3.0, 1.0);
        //        v.make_unit_vector();
        //        assert_eq!(v, Vec3::new(1.0, 1.0, 1.0))
    }
}
