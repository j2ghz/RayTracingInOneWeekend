use crate::{ray::Ray, vec3d::Vec3d};
pub mod sphere;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3d,
    pub normal: Vec3d,
}
pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
pub fn get_hits(vec: &[Box<dyn Hitable>], r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    vec.iter()
        .filter_map(|h| h.hit(r, t_min, t_max))
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}
