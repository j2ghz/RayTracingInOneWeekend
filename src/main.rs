use image::{ImageBuffer, ImageFormat, RgbImage};
use rand::Rng;
use raytracing_in_one_weekend::{
    camera::Camera,
    color::Rgb,
    geometry::{get_hits, Hitable},
    ray::Ray,
    vec3d::Vec3d,
};
use std::time::Instant;

fn get_color(r: Ray, hitables: &[Box<dyn Hitable>]) -> Rgb {
    if let Some(hit_rec) = get_hits(hitables, r, 0.001, std::f64::MAX) {
        let target = hit_rec.p + hit_rec.normal + Vec3d::random_in_unit_sphere();
        get_color(Ray::new(hit_rec.p, target - hit_rec.p), hitables) * 0.5
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = (unit_direction.y() + 1.0) * 0.5;
        Rgb::new(1.0, 1.0, 1.0).blend(Rgb::new(0.5, 0.7, 1.0), t)
    }
}

fn main() {
    let start = Instant::now();
    let w = 2000;
    let h = 1000;
    let samples = 1000;
    let mut img: RgbImage = ImageBuffer::new(w, h);

    let cam = Camera::default();

    let objects: Vec<Box<dyn Hitable>> = vec![
        Box::new(raytracing_in_one_weekend::geometry::sphere::Sphere {
            center: Vec3d::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(raytracing_in_one_weekend::geometry::sphere::Sphere {
            center: Vec3d::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];
    let mut rng = rand::thread_rng();
    for j in (0..(h - 1)).rev() {
        for i in 0..(w - 1) {
            let mut color = Rgb::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f64 + rng.gen::<f64>()) / w as f64;
                let v = (j as f64 + rng.gen::<f64>()) / h as f64;
                let ray = cam.get_ray(u, v);

                color += get_color(ray, &objects);
            }
            let color = color / samples as f64;
            let y = (h - 1) - j;
            let x = i;
            //println!("x:{} y:{} color:{:?}",x,y,color);
            img.put_pixel(x, y, color.into());
        }
    }

    let duration = start.elapsed();
    println!("Rendered in: {:?}", duration);

    img.save_with_format("./target/out.bmp", ImageFormat::Bmp)
        .expect("Couldn't save image");
}
