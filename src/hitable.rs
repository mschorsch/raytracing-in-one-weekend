use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<Material>,
}

impl Hit {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: Box<Material>) -> Self {
        Hit {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material>,
}

impl Sphere {
    pub fn new<M: Material + 'static>(center: Vec3, radius: f32, material: M) -> Self {
        Sphere {
            center,
            radius,
            material: Box::new(material),
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        // ax^2 + bx + c = 0
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&(ray.origin - self.center));
        let c = (ray.origin - self.center).dot(&(ray.origin - self.center))
            - (self.radius * self.radius);
        let discriminant: f32 = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a); // midnight formula => wrong in pdf
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius; // normal unit vector?
                return Some(Hit::new(t, p, normal, self.material.box_clone()));
            }

            let temp = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a); // midnight formula => wrong in pdf
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius; // normal unit vector?
                return Some(Hit::new(t, p, normal, self.material.box_clone()));
            }
        }
        None
    }
}

pub struct World {
    pub hitables: Vec<Box<Hitable>>,
}

impl World {
    pub fn new(hitables: Vec<Box<Hitable>>) -> Self {
        World { hitables }
    }
}

impl Hitable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;
        for hitable in &self.hitables {
            if let Some(hit) = hitable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;

    #[test]
    fn test_world() {
        let s1 = Sphere::new(
            Vec3(0.0, 0.0, 0.0),
            1.0,
            Lambertian::new(Vec3(1.0, 0.0, 0.0)),
        );
        let s2 = Sphere::new(
            Vec3(0.0, 0.0, 0.0),
            2.0,
            Lambertian::new(Vec3(1.0, 0.0, 0.0)),
        );
        let hitables: Vec<Box<Hitable>> = vec![Box::new(s1), Box::new(s2)];
        let world = World::new(hitables);

        let r = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, -1.0));
        assert_eq!(world.hit(&r, 0.0, 1.0), None);
    }
}
