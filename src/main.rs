use image::{ImageBuffer, ImageFormat, RgbImage};

fn main() {
    println!("Hello, world!");
    let mut img: RgbImage = ImageBuffer::new(200, 100);
    for x in 0..200 {
        for y in 0..100 {
            img.put_pixel(x, y, [x as u8, 200 - (y * 2) as u8, 0].into());
        }
    }
    img.save_with_format("out.bmp", ImageFormat::Bmp)
        .expect("Couldn't save image");
}
