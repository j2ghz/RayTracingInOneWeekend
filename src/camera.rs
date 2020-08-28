use crate::{ray::Ray, vec3d::Vec3d};

pub struct Camera {
    origin: Vec3d,
    lower_left_corner: Vec3d,
    horizontal_size: Vec3d,
    vertical_size: Vec3d,
}
impl Default for Camera {
    fn default() -> Self {
        Camera {
            lower_left_corner: Vec3d::new(-2.0, -1.0, -1.0),
            horizontal_size: Vec3d::new(4.0, 0.0, 0.0),
            vertical_size: Vec3d::new(0.0, 2.0, 0.0),
            origin: Vec3d::new(0.0, 0.0, 0.0),
        }
    }
}
impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal_size * u + self.vertical_size * v
                - self.origin,
        )
    }
}
