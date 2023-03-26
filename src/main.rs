use std::fmt::Write;

use ray_tracer::write_to_file;

const IMAGE_WIDTH: i32 = 256 * 4;
const IMAGE_HEIGHT: i32 = 256 * 4;
const MAX_COLOR: i32 = 255;

fn main() {
    let mut image_out = format!("P3\n{IMAGE_WIDTH}\n{IMAGE_WIDTH}\n{MAX_COLOR}\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            writeln!(&mut image_out, "{ir} {ig} {ib}").expect("Couldn't write to string buffer");
        }
    }

    write_to_file(image_out).expect("Couldn't write to file");
}
