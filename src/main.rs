use rand::{random, thread_rng, Rng};
use ray_tracer::camera::Camera;
use ray_tracer::hittable::HittableList;
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::vec3::{Color, Point3, Vec3};
use ray_tracer::{ray_color, write_color_to_str, write_string_to_file};
use std::rc::Rc;

// Image
const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_WIDTH: i32 = 256 * 4;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const MAX_COLOR: i32 = 255;

// Camera
const SAMPLES_PER_PIXEL: i32 = 100;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera Setup
    let camera = Camera::default();

    let mut image_out = format!("P3\n{IMAGE_WIDTH}\n{IMAGE_HEIGHT}\n{MAX_COLOR}\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0., 0., 0.);
            for k in 0..SAMPLES_PER_PIXEL {
                // Anti-aliasing
                let u = (i as f32 + random::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random::<f32>()) / (IMAGE_HEIGHT - 1) as f32;

                // // Uncomment if sample size is 1
                // let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
                // let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            write_color_to_str(&mut image_out, pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    write_string_to_file(image_out).expect("Couldn't write to file");

    println!("Done");
}
