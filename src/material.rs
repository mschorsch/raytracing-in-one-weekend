use crate::hitable::Hit;
use crate::rand::prelude::*;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

use std::fmt::Debug;

#[derive(Debug)]
pub struct Scatter {
    // Dämpfung
    pub attenuation: Vec3,
    // Streuung
    pub scattered: Ray,
}

pub trait Material: Debug {
    fn scatter(&self, ray_in: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Scatter>;
    fn box_clone(&self) -> Box<Material>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lambertian /* diffuse */ {
    albedo: Vec3, // Rückstrahlungsvermögen
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Scatter> {
        let target = hit.p + hit.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(hit.p, target - hit.p);
        let attenuation = self.albedo;
        Some(Scatter {
            scattered,
            attenuation,
        })
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(Self::new(self.albedo))
    }
}

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p =
            2.0 * Vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metal {
    // Rückstrahlungsvermögen
    albedo: Vec3,
    // size of the "reflection" sphere
    fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f32) -> Self {
        assert!(fuzziness >= 0.0 && fuzziness <= 1.0);
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Scatter> {
        let reflected = reflect(&unit_vector(ray_in.direction), &hit.normal);
        let scattered = Ray::new(
            hit.p,
            reflected + self.fuzziness * random_in_unit_sphere(rng),
        );
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some(Scatter {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(Self::new(self.albedo, self.fuzziness))
    }
}

fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(normal) * normal
}
