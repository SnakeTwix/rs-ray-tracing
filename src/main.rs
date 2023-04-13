use rand::random;
use ray_tracer::camera::Camera;
use ray_tracer::hittable::HittableList;
use ray_tracer::material::{Dielectric, Lambertian, Metal};
use ray_tracer::sphere::Sphere;
use ray_tracer::vec3::{Color, Point3};
use ray_tracer::{ray_color, write_color_to_str, write_string_to_file};
use std::rc::Rc;

// Image
const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: i32 = 1080;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const MAX_COLOR: i32 = 255;
const MAX_DEPTH: i32 = 100;

// Camera
const SAMPLES_PER_PIXEL: i32 = 4000;

fn main() {
    // World
    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    // let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    let material_right = Rc::new(Dielectric::new(1.));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.4,
        material_right,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-2., 0., 5.),
        0.5,
        material_center,
    )));

    // Camera Setup
    let camera = Camera::default();

    let mut image_out = format!("P3\n{IMAGE_WIDTH}\n{IMAGE_HEIGHT}\n{MAX_COLOR}\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                // Anti-aliasing
                let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                // // Uncomment if sample size is 1
                // let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
                // let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            write_color_to_str(&mut image_out, pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    write_string_to_file(image_out).expect("Couldn't write to file");

    println!("Done");
}
