use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> io::Result<()> {
    let mut file = BufWriter::with_capacity(width * height * 3, File::create(file_path)?);
    file.write(format!("P6\n{} {} 255\n", width, height).as_bytes())?;
    for y in 0..width {
        for x in 0..height {
            let pixel = pixels[y * width + x];
            let color = [
                ((pixel >> 8 * 2) & 0xFF) as u8, // returns red color
                ((pixel >> 8 * 1) & 0xFF) as u8, // returns green color
                ((pixel >> 8 * 0) & 0xFF) as u8  // returns blue color
            ];
            file.write(&color)?;
        }
    }
    println!("Created {}", file_path);
    Ok(())
}

fn draw_check_pattern(pixels: &mut [u32], foreground_color: u32, background_color: u32,  width: usize, height: usize) {
    for y in 0..width {
        for x in 0..height {
            pixels[y * width + x] = if x % 2 == y % 2 {
                foreground_color
            } else {
                background_color
            }
        }
    }
}

fn main() {
    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;
    const BACKGROUND_COLOR: u32 = 0x000000;
    const FOREGROUNG_COLOR: u32 = 0xFF00FF;
    let mut pixels = [FOREGROUNG_COLOR; WIDTH * HEIGHT];
    draw_check_pattern(&mut pixels, FOREGROUNG_COLOR, BACKGROUND_COLOR, WIDTH, HEIGHT);
    save_as_ppm("test.ppm", &pixels, WIDTH, HEIGHT).unwrap();
}
