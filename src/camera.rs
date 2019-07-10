use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

use rand::prelude::*;
use std::convert::TryInto;
use std::f32::consts::PI;

struct OrthoPlane {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,

    plane: OrthoPlane,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3, /* vertical up vector */
        vfov: f32, /* vertical field of view in degrees */
        aspect: f32,
        aperture: f32,   /* size of the lense */
        focus_dist: f32, /* distance */
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan(); // Berehnung auf dem Einheitskreis
        let half_width = aspect * half_height;

        let origin = lookfrom;
        // orthonarmal basis
        let w = unit_vector(lookfrom - lookat); // -w!!!
        let u = unit_vector(vup.cross(&w));
        let v = w.cross(&u); // w x u because we have -w and a x b = -b x a

        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        let lens_radius = aperture / 2.0;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            plane: OrthoPlane { u, v, w },
            lens_radius,
        }
    }

    pub fn get_ray(&self, rng: &mut ThreadRng, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.plane.u * rd.x() + self.plane.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p = 2.0 * Vec3(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - Vec3(1.0, 1.0, 0.0);
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}
