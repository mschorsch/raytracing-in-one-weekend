use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn r(&self) -> f32 {
        self.0
    }

    pub fn g(&self) -> f32 {
        self.1
    }

    pub fn b(&self) -> f32 {
        self.2
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        // scalar product
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        // cross product
        let x = self.1 * other.2 - self.2 * other.1;
        let y = -(self.0 * other.2 - self.2 * other.0);
        let z = self.0 * other.1 - self.1 * other.0;
        Vec3(x, y, z)
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1_f32 / self.length();
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
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
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
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
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
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
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl<'a> ops::Mul<&'a Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &'a Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
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
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}

impl<'a> ops::AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &'a Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<'a> ops::SubAssign<&'a Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &'a Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl<'a> ops::MulAssign<&'a Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &'a Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

impl<'a> ops::DivAssign<&'a Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: &'a Vec3) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        let k = 1_f32 / t;
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
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
