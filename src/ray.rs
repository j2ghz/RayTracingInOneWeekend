use super::vec3d::Vec3d;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    /// Origin vector
    a: Vec3d,
    /// Direction
    b: Vec3d,
}
impl Ray {
    pub fn new(origin: Vec3d, direction: Vec3d) -> Ray {
        Ray {
            a: origin,
            b: direction,
        }
    }
    pub fn origin(self) -> Vec3d {
        self.a
    }
    pub fn direction(self) -> Vec3d {
        self.b
    }
    pub fn point_at_parameter(self, t: f64) -> Vec3d {
        self.a + (self.b * t)
    }
}
