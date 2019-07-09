use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

use std::f32::consts::PI;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3, /* vertical up vector */
        vfov: f32, /* vertical field of view in degrees */
        aspect: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan(); // Berehnung auf dem Einheitskreis
        let half_width = aspect * half_height;

        let origin = lookfrom;
        // orthonarmal basis
        let w = unit_vector(lookfrom - lookat); // -w!!!
        let u = unit_vector(vup.cross(&w));
        let v = w.cross(&u); // w x u because we have -w and a x b = -b x a

        // let lower_left_corner = Vec3(-half_width, -half_height, -1.0);
        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
