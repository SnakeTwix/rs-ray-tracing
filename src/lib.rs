pub mod camera;
pub mod hittable;
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
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f32;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    writeln!(
        str,
        "{:.0} {:.0} {:.0}",
        255. * r.clamp(0., 1.),
        255. * g.clamp(0., 1.),
        255. * b.clamp(0., 1.),
    )
    .expect("Couldn't write to string buffer");
}

pub fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0., f32::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + random_unit_vector();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0..1.0);
        if p.length_squared() >= 1. {
            continue;
        }

        return p;
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_num_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * fastrand::f32()
}
