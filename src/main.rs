use image::{ImageBuffer, ImageFormat, RgbImage};
use raytracing_in_one_weekend::color::Rgb;

fn main() {
    println!("Hello, world!");
    let mut img: RgbImage = ImageBuffer::new(200, 100);
    for x in 0..200 {
        for y in 0..100 {
            let color = Rgb::new((x as f64 / 200.0).into(), (y as f64 / 100.0).into(), 0.2f64);
            img.put_pixel(x, y, color.into());
        }
    }
    img.save_with_format("./target/out.bmp", ImageFormat::Bmp)
        .expect("Couldn't save image");
}
