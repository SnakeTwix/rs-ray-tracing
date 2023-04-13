pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use std::fmt::Write as _;
use std::{fs::File, io::Write as _};

pub fn write_string_to_file(str: String) -> std::io::Result<()> {
    let mut f = File::create("image.ppm")?;
    f.write_all(str.as_bytes())?;
    Ok(())
}

pub fn write_color_to_str(str: &mut String, pixel_color: Color, samples_per_pixel: i32) {
    let scaled = pixel_color / samples_per_pixel as f64;
    let gamma_adjusted = Color::new(scaled.x.sqrt(), scaled.y.sqrt(), scaled.z.sqrt());

    writeln!(
        str,
        "{:.0} {:.0} {:.0}",
        255. * gamma_adjusted.x.clamp(0., 1.),
        255. * gamma_adjusted.y.clamp(0., 1.),
        255. * gamma_adjusted.z.clamp(0., 1.),
    )
    .expect("Couldn't write to string buffer");
}

pub fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        let material = rec.material.clone();

        if material.scatter(r, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::new(0., 0., 0.);
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0..1.0);
        if p.length_squared() < 1. {
            return p;
        }
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2. * v.dot(&normal) * normal
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(&n).min(-1.);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1. - r_out_perp.length_squared()).abs().sqrt() * n;

    r_out_perp + r_out_parallel
}
