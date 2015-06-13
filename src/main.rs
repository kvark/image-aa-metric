extern crate image;

fn main() {
    type Count = u64;
    use std::env;
    use std::path::Path;
    use image::GenericImage;

    let file = env::args()
                   .nth(1)
                   .expect("Please enter a file");

    println!("Opening {}", file);
    let im = image::open(&Path::new(&file))
                   .unwrap()
                   .to_rgb();

    let (width, height) = im.dimensions();
    println!("Dimensions are {}x{}", width, height);

    if width < 2 || height < 2 {
        return;
    }

    fn get_score(a: u8, b: u8, x: u8) -> f32 {
        let sigma = ((a as i32) - (b as i32)).abs();
        let avg = ((a as i32) + (b as i32)) >> 1;
        let smoother = 5f32;
        let divisor = smoother / (sigma as f32 + smoother);
        let relative = (x as i32 - avg).abs() as f32 * divisor;
        let base = 2f32;
        (relative - base).max(0.0)
    }

    let mut score = [0f32; 3];

    for y in 1..height {
        for x in 1..width {
            let a = im.get_pixel(x-1, y-1);
            let b = im.get_pixel(x, y-1);
            let c = im.get_pixel(x-1, y);
            let d = im.get_pixel(x, y);
            for k in 0..3 {
                score[k] += get_score(b.data[k], c.data[k], a.data[k]);
                score[k] += get_score(b.data[k], c.data[k], d.data[k]);
                score[k] += get_score(a.data[k], d.data[k], b.data[k]);
                score[k] += get_score(a.data[k], d.data[k], c.data[k]);
            }
        }
    }

    let total = (width - 1) * (height - 1);
    let sum = score.iter().fold(0f32, |u, &s| u + s) / (total as f32);
    println!("AA metric score: {:2}", sum);
}
