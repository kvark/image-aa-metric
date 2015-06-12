extern crate image;

fn main() {
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
    
    let mut sum1_hor = [0u32; 3];
    let mut sum2_hor = [0u32; 3];
    let mut sum1_ver = [0u32; 3];
    let mut sum2_ver = [0u32; 3];

    for y in 0..height {
        for x in 0..width {
            let pixel = im.get_pixel(x, y);
            if x > 0 {
                let prev = im.get_pixel(x-1, y);
                for k in 0..3 {
                    let diff = ((pixel.data[k] as i32) - (prev.data[k] as i32)).abs() as u32;
                    sum1_hor[k] += diff;
                    sum2_hor[k] += diff*diff;
                }
            }
            if y > 0 {
                let prev = im.get_pixel(x, y-1);
                for k in 0..3 {
                    let diff = ((pixel.data[k] as i32) - (prev.data[k] as i32)).abs() as u32;
                    sum1_ver[k] += diff;
                    sum2_ver[k] += diff*diff;
                }
            }
        }
    }

    fn print_metric(data: [u32; 3], total: u32, power: f32, name: &str) {
        let mut rel = [0f32; 3];
        for (r, d) in rel.iter_mut().zip(data.iter()) {
            *r =  ((*d as f32) / (total as f32)).powf(power);
        }
        println!("{}: {:2}, {:2}, {:2}", name, rel[0], rel[1], rel[2]);
    }

    let total_hor = (width - 1) * height;
    let total_ver = width * (height - 1);

    println!("AA metrics:");
    print_metric(sum1_hor, total_hor, 1.0, "Horisontal pow1");
    print_metric(sum2_hor, total_hor, 0.5, "Horisontal pow2");
    print_metric(sum1_ver, total_ver, 1.0, "Vertical pow1");
    print_metric(sum2_ver, total_ver, 0.5, "Vertical pow2");
}
