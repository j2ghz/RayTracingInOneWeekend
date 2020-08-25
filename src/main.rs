use image::{ImageBuffer, ImageFormat, RgbImage};
use raytracing_in_one_weekend::{color::Rgb, ray::Ray, vec3d::Vec3d};

fn hit_sphere(center: Vec3d, radius: f64, r: Ray) -> Option<f64> {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = oc.dot(r.direction()) * 2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;
    if discriminant < 0.0 {
        None
    } else {
        Some(-b - discriminant.sqrt() / 2.0 * a)
    }
}

fn color(r: Ray) -> Rgb {
    if let Some(t) = hit_sphere(Vec3d::new(0.0, 0.0, -1.0), 0.5, r) {
        let normal = (r.point_at_parameter(t) - Vec3d::new(0.0, 0.0, -1.0)).unit_vector();
        Rgb::new(
            (normal.x() + 1.0) * 0.5,
            (normal.y() + 1.0) * 0.5,
            (normal.z() + 1.0) * 0.5,
        )
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = (unit_direction.y() + 1.0) * 0.5;
        Rgb::new(t, t, t)
    }
}

fn main() {
    println!("Hello, world!");
    let w = 200;
    let h = 100;
    let mut img: RgbImage = ImageBuffer::new(w, h);

    let lower_left_corner = Vec3d::new(-2.0, -1.0, -1.0);
    let horizontal_size = Vec3d::new(4.0, 0.0, 0.0);
    let vertical_size = Vec3d::new(0.0, 2.0, 0.0);
    let origin = Vec3d::new(0.0, 0.0, 0.0);

    for j in (0..(h - 1)).rev() {
        for i in 0..(w - 1) {
            let u = i as f64 / w as f64;
            let v = j as f64 / h as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal_size * u + vertical_size * v,
            );

            let color = color(ray);
            let y = (h - 1) - j;
            let x = i;
            //println!("x:{} y:{} color:{:?}",x,y,color);
            img.put_pixel(x, y, color.into());
        }
    }
    img.save_with_format("./target/out.bmp", ImageFormat::Bmp)
        .expect("Couldn't save image");
}
