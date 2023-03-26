pub mod vec3;

use crate::vec3::Color;
use std::fmt::Write as StrWriter;
use std::{fs::File, io::Write};

pub fn write_to_file(str: String) -> std::io::Result<()> {
    let mut f = File::create("image.ppm")?;
    f.write_all(str.as_bytes())?;
    Ok(())
}

pub fn write_color_to_str(str: &mut String, pixel_color: Color) {
    let ir = (pixel_color.x * 255.999) as i32;
    let ig = (pixel_color.y * 255.999) as i32;
    let ib = (pixel_color.z * 255.999) as i32;

    writeln!(str, "{ir} {ig} {ib}").expect("Couldn't write to string buffer");
}
