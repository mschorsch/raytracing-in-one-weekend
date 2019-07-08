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
    pub fn new(albedo: Vec3) -> Self {
        Self::with_fuzziness(albedo, 0.0)
    }

    pub fn with_fuzziness(albedo: Vec3, fuzziness: f32) -> Self {
        assert!(fuzziness >= 0.0 && fuzziness <= 1.0);
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Scatter> {
        let reflected = reflect(unit_vector(ray_in.direction), &hit.normal);
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
        Box::new(Self::with_fuzziness(self.albedo, self.fuzziness))
    }
}

fn reflect(v: Vec3, normal: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Scatter> {
        let outward_normal;
        let ni_over_nt: f32;
        let cosine: f32;
        if ray_in.direction.dot(&hit.normal) > 0.0 {
            // spitzer Winkel!
            outward_normal = -hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray_in.direction.dot(&hit.normal) / ray_in.direction.length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -ray_in.direction.dot(&hit.normal) / ray_in.direction.length();
        }

        let scattered: Ray;
        if let Some(refracted) = refract(ray_in.direction, &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_idx);

            if rng.gen::<f32>() < reflect_prob {
                let reflected = reflect(ray_in.direction, &hit.normal); // FIXME why is direction not a normal vector???
                scattered = Ray::new(hit.p, reflected);
            } else {
                scattered = Ray::new(hit.p, refracted);
            }
        } else {
            let reflected = reflect(ray_in.direction, &hit.normal); // FIXME why is direction not a normal vector???
            scattered = Ray::new(hit.p, reflected);
        }

        let attenuation = Vec3(1.0, 1.0, 1.0);

        Some(Scatter {
            scattered,
            attenuation,
        })
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(Self::new(self.ref_idx))
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx); // FIXME what? should be (ref_idx - 1.0)????
    let r0 = r0 * r0; // see fundamentals in graphics p.325
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(v: Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> /* refracted */ {
    let uv = unit_vector(v);
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt); // FIXME should be b*b-4.0*a* c???

    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - *normal * dt) - *normal * discriminant.sqrt(); //??
        Some(refracted)
    } else {
        None
    }
}
