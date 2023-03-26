use ray_tracer::vec3::Color;
use ray_tracer::{write_color_to_str, write_to_file};

const IMAGE_WIDTH: i32 = 256 * 4;
const IMAGE_HEIGHT: i32 = 256 * 4;
const MAX_COLOR: i32 = 255;

fn main() {
    let mut image_out = format!("P3\n{IMAGE_WIDTH}\n{IMAGE_WIDTH}\n{MAX_COLOR}\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f32 / (IMAGE_WIDTH - 1) as f32,
                j as f32 / (IMAGE_HEIGHT - 1) as f32,
                0.25,
            );

            write_color_to_str(&mut image_out, pixel_color);
        }
    }

    write_to_file(image_out).expect("Couldn't write to file");

    println!("Done");
}
