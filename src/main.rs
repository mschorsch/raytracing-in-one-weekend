extern crate rand;

use std::fs;
use std::io;
use std::io::prelude::*;

use rand::prelude::*;

use crate::camera::Camera;
use crate::hitable::{Hitable, Sphere, World};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::vec3::Vec3;

mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;

// image size
const NX: u32 = 1200;
const NY: u32 = 800;
const NS: u32 = 100;

fn main() -> io::Result<()> {
    let mut file = fs::File::create("image.ppm")?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", NX, NY)?;
    writeln!(file, "255")?;

    let mut rng = thread_rng(); // random number generator; standard distribution [0, 1)

    // Camera
    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0; //(lookfrom - lookat).length();
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3(0.0, 1.0, 0.0),
        20.0,
        NX as f32 / NY as f32,
        aperture,
        dist_to_focus,
    );

    // World
    let world = random_scene(&mut rng);

    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..NS {
                let u = (i as f32 + rng.gen::<f32>()/* 0 <= x < 1 */) / NX as f32;
                let v = (j as f32 + rng.gen::<f32>()/* 0 <= x < 1 */) / NY as f32;
                let ray = camera.get_ray(&mut rng, u, v);

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

fn random_scene(rng: &mut ThreadRng) -> World {
    let n = 500;
    let mut list: Vec<Box<Hitable>> = Vec::with_capacity(n + 1);
    list.push(
        Sphere::new(
            Vec3(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(Vec3(0.5, 0.5, 0.5)),
        )
        .into_box(),
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3(
                a as f32 + rng.gen::<f32>() * 0.9,
                0.2,
                b as f32 + rng.gen::<f32>() * 0.9,
            );
            if (center - Vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    /* diffuse */
                    list.push(
                        Sphere::new(
                            center,
                            0.2,
                            Lambertian::new(Vec3(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            )),
                        )
                        .into_box(),
                    );
                } else if choose_mat < 0.95 {
                    /* metal */
                    list.push(
                        Sphere::new(
                            center,
                            0.2,
                            Metal::with_fuzziness(
                                Vec3(
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                ),
                                0.5 * rng.gen::<f32>(),
                            ),
                        )
                        .into_box(),
                    );
                } else {
                    /* glass */
                    list.push(Sphere::new(center, 0.2, Dielectric::new(1.5)).into_box());
                }
            }
        }
    }

    list.push(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)).into_box());
    list.push(
        Sphere::new(
            Vec3(-4.0, 1.0, 0.0),
            1.0,
            Lambertian::new(Vec3(0.4, 0.2, 0.1)),
        )
        .into_box(),
    );
    list.push(Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, Metal::new(Vec3(0.7, 0.6, 0.5))).into_box());

    World::new(list)
}

fn color(ray: &Ray, world: &World, rng: &mut ThreadRng, depth: u32) -> Vec3 {
    use std::f32;

    // 0.001; ignore hits very near zero
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
        let unit_direction = (ray.direction).normalized(); // unit vector -1 <= x <= 1
        let t = 0.5 * (unit_direction.y() + 1.0); // scale to 0 <= x <= 1
        (1.0 - t) * Vec3(1.0, 1.0, 1.0) /* white */ + t * Vec3(0.5, 0.7, 1.0) /* blue */
    }
}
