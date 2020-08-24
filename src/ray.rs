use super::vec3d::Vec3d;
use num::Float;

pub struct Ray<F: Float> {
    /// Origin vector
    a: Vec3d<F>,
    /// Direction
    b: Vec3d<F>,
}
impl<F: Float> Ray<F> {
    fn new(origin: Vec3d<F>, direction: Vec3d<F>) -> Ray<F> {
        Ray {
            a: origin,
            b: direction,
        }
    }
    fn origin(self) -> Vec3d<F> {
        self.a
    }
    fn direction(self) -> Vec3d<F> {
        self.b
    }
    fn point_at_parameter(self, t: F) -> Vec3d<F> {
        self.a + (self.b * t)
    }
}
