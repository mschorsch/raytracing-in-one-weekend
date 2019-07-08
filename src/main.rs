#![allow(dead_code)]
#![allow(unused_imports)]

extern crate rand;

use std::fs;
use std::io;
use std::io::prelude::*;

use rand::prelude::*;

use crate::camera::Camera;
use crate::hitable::{Hit, Hitable, Sphere, World};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;

// image size
const NX: u32 = 200;
const NY: u32 = 100;
const NS: u32 = 100;

fn main() -> io::Result<()> {
    let mut file = fs::File::create("image.ppm")?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", NX, NY)?;
    writeln!(file, "255")?;

    let mut rng = thread_rng(); // random number generator; standard distribution [0, 1)

    let world = World::new(vec![
        Box::new(Sphere::new(
            Vec3(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Vec3(0.1, 0.2, 0.5)),
        )),
        Box::new(Sphere::new(
            Vec3(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Vec3(0.8, 0.8, 0.0)),
        )),
        Box::new(Sphere::new(
            Vec3(1.0, 0.0, -1.0),
            0.5,
            Metal::new(Vec3(0.8, 0.6, 0.2)),
        )),
        Box::new(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            0.5,
            Dielectric::new(1.5),
        )),
        Box::new(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            -0.45,
            Dielectric::new(1.5),
        )),
    ]);

    let camera = Camera::new();

    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..NS {
                let u = (i as f32 + rng.gen::<f32>()/* 0 <= x < 1 */) / NX as f32;
                let v = (j as f32 + rng.gen::<f32>()/* 0 <= x < 1 */) / NY as f32;
                let ray = camera.get_ray(u, v);

                // let p = r.point_at_parameter(2.0); // 2.0??
                col += &color(&ray, &world, &mut rng, 0);
            }
            col /= NS as f32;
            col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt()); // gamma correction "gamma 2"

            let ir = (255.99 * col.0) as u32;
            let ig = (255.99 * col.1) as u32;
            let ib = (255.99 * col.2) as u32;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}

fn color(ray: &Ray, world: &World, rng: &mut ThreadRng, depth: u32) -> Vec3 {
    use std::f32;

    // 0.001; ignore heats very near zero
    if let Some(hit) = world.hit(ray, 0.001, f32::MAX) {
        if depth >= 50 {
            return Vec3(0.0, 0.0, 0.0);
        }

        if let Some(scatter) = hit.material.scatter(ray, &hit, rng) {
            scatter.attenuation * color(&scatter.scattered, world, rng, depth + 1)
        } else {
            Vec3(0.0, 0.0, 0.0)
        }
    } else {
        // linear blend; linear interpolation; lerp
        // blended_value = (1-t)*start_value + t*end_value
        let unit_direction = unit_vector(ray.direction); // unit vector -1 <= x <= 1
        let t = 0.5 * (unit_direction.y() + 1.0); // scale to 0 <= x <= 1
        (1.0 - t) * Vec3(1.0, 1.0, 1.0) /* white */ + t * Vec3(0.5, 0.7, 1.0) /* blue */
    }
}
