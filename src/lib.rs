pub mod camera;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Color;
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
    r *= scale;
    g *= scale;
    b *= scale;

    writeln!(
        str,
        "{:.0} {:.0} {:.0}",
        255. * r.clamp(0., 1.),
        255. * g.clamp(0., 1.),
        255. * b.clamp(0., 1.),
    )
    .expect("Couldn't write to string buffer");
}

pub fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0., f32::INFINITY, &mut rec) {
        // dbg!(rec.normal);
        return 0.5 * (rec.normal + Color::new(1., 1., 1.));
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
