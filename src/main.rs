use std::fs::File;
use std::io::prelude::*;
use std::io;

// TODO: create a ppm file with p6 magic number
// TODO: rewrite the function with buffer
fn save_as_ppm(file_path: &str, pixels: &[usize], width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write(format!("P3\n{} {} 255\n", width, height).as_bytes())?;
    for y in 0..width {
        for x in 0..height {
            file.write(format!("{} {} {}\n",
                              ((pixels[y * width + x] >> 16) & 0xFF),
                              ((pixels[y * width + x] >> 8) & 0xFF),
                              ((pixels[y * width + x] >> 0) & 0xFF)).as_bytes()
            )?;
        }
    }
    println!("Created {}", file_path);
    Ok(())
}

fn main() {
    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;
    const BACKGROUND_COLOR: usize = 0x000000;
    const FOREGROUNG_COLOR: usize = 0xFF00FF;
    let mut pixels = [FOREGROUNG_COLOR; WIDTH * HEIGHT];
    save_as_ppm("test.ppm", &pixels, WIDTH, HEIGHT);
}
