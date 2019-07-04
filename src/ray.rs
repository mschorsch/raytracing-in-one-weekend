use crate::vec3::Vec3;

// half-line or ray
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    // p(t) = a + t*b
    // p: point; a: origin; b: direction
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction } // FIXME copy
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        // p(t) = a + t*b
        self.origin + t * self.direction
    }
}
