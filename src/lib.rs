use std::{fs::File, io::Write};

pub fn write_to_file(str: String) -> std::io::Result<()> {
    let mut f = File::create("image.ppm")?;
    f.write_all(str.as_bytes())?;
    Ok(())
}
