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

fn draw_check_pattern(pixels: &mut [u32], foreground_color: u32, background_color: u32,  width: usize, height: usize, tile_size: usize) {
    for y in 0..width {
        for x in 0..height {
            pixels[y * width + x] = if (x / tile_size + y / tile_size) % 2 == 0 {
                foreground_color
            } else {
                background_color
            };
        }
    }
}

fn draw_stripe_pattern(pixels: &mut [u32], foreground_color: u32, background_color: u32, width: usize, height: usize, tile_size: usize) {
    for y in 0..width {
        for x in 0..height {
            pixels[y * width + x] = if ((x + y) / tile_size) % 2 == 0 {
                foreground_color
            } else {
                background_color
            }
        }
    }
}

fn draw_solid_circle(pixels: &mut [u32], foreground_color: u32, background_color: u32,  width: usize, height: usize, radius: usize) {
    let cx = width as i32;
    let cy = height as i32;
    let r = radius as i32 * 2;
    for y in 0..width {
        for x in 0..height {
            let dx = cx - x as i32 * 2 - 1;
            let dy = cy - y as i32 * 2 - 1;
            pixels[y * width + x] = if dx*dx + dy*dy <= r*r {
                foreground_color
            } else {
                background_color
            };
        }
    }
}
// Uses midpoint circle algorithm
// done with integer subpixel computations like in solid circle
fn draw_hollow_circle(pixels: &mut [u32], foreground_color: u32, background_color: u32, width: usize, height: usize, radius: usize) {
    pixels.fill(background_color);

    let w = width * 2;
    let h = height * 2;
    let r = radius * 2;
    let cx = w / 2;
    let cy = h / 2;
    let mut x = 0;
    let mut y = r - 1;

    while x <= y {
        let px = x + cx;
        let py = y + cy;
        if (0..w).contains(&px) && (0..h).contains(&py) {
            assert!(width == height);
            let dx = px / 2;
            let dy = py / 2;

            pixels[dy * width + dx] = foreground_color;
            pixels[dx * width + dy] = foreground_color;

            pixels[(height - dy) * width + dx] = foreground_color;
            pixels[dx * width + (height - dy)] = foreground_color;

            pixels[dy * width + (width - dx)] = foreground_color;
            pixels[(width - dx) * width + dy] = foreground_color;

            pixels[(height - dy) * width + (width - dx)] = foreground_color;
            pixels[(width - dx) * width + (height - dy)] = foreground_color;
        }

        x += 2;
        if x*x + y*y > r*r {
            y -= 2;
        }
    }
}

fn draw_vertical_stripes(pixels: &mut [u32], foreground_color: u32, background_color: u32, width: usize, height: usize, tile_size: usize) {
    for y in 0..width {
        for x in 0..height {
            pixels[y * width + x] = if x % (tile_size * 2) < tile_size {
                foreground_color
            } else {
                background_color
            }
        }
    }
}

fn draw_horizontal_stripes(pixels: &mut [u32], foreground_color: u32, background_color: u32, width: usize, height: usize, tile_size: usize) {
    for y in 0..width {
        for x in 0..height {
            pixels[y * width + x] = if y % (tile_size * 2) < tile_size {
                foreground_color
            } else {
                background_color
            }
        }
    }
}

fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;
    const RADIUS: usize = WIDTH / 2;
    const TILE_SIZE: usize = WIDTH / 16;
    const BACKGROUND_COLOR: u32 = 0x000000;
    const FOREGROUNG_COLOR: u32 = 0xFF00FF;
    let mut pixels = [FOREGROUNG_COLOR; WIDTH * HEIGHT];


    pixels.fill(0x00FF00);
    draw_check_pattern(&mut pixels, FOREGROUNG_COLOR, BACKGROUND_COLOR, WIDTH, HEIGHT, TILE_SIZE);
    save_as_ppm("check_pattern.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    pixels.fill(0x00FF00);
    draw_stripe_pattern(&mut pixels,FOREGROUNG_COLOR, BACKGROUND_COLOR,WIDTH, HEIGHT, TILE_SIZE);
    save_as_ppm("stripe_pattern.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    pixels.fill(0x00FF00);
    draw_solid_circle(&mut pixels, FOREGROUNG_COLOR, BACKGROUND_COLOR, WIDTH, HEIGHT, RADIUS);
    save_as_ppm("solid_circle.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    pixels.fill(0x00FF00);
    draw_hollow_circle(&mut pixels, FOREGROUNG_COLOR, BACKGROUND_COLOR, WIDTH, HEIGHT, RADIUS);
    save_as_ppm("hollow_circle.ppm", &pixels, WIDTH, HEIGHT).unwrap();
    pixels.fill(0x00FF00);
    draw_vertical_stripes(&mut pixels, FOREGROUNG_COLOR, BACKGROUND_COLOR, WIDTH, HEIGHT, TILE_SIZE);
    save_as_ppm("vertical_stripes.ppm", &pixels, WIDTH, HEIGHT).unwrap();
    pixels.fill(0x00FF00);
    draw_horizontal_stripes(&mut pixels, FOREGROUNG_COLOR, BACKGROUND_COLOR, WIDTH, HEIGHT, TILE_SIZE);
    save_as_ppm("horizontal_stripes.ppm", &pixels, WIDTH, HEIGHT).unwrap();
}
