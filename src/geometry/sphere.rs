use super::{HitRecord, Hitable};
use crate::{ray, vec3d::Vec3d};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3d,
    pub radius: f64,
}
impl Hitable for Sphere {
    fn hit(&self, r: ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot_with_self();
        let b = oc.dot(r.direction());
        let c = oc.dot_with_self() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let temp = (-b - ((b.powi(2) - a * c).sqrt())) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                });
            }
            let temp = (-b + (b.powi(2) - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                });
            }
        }
        None
    }
}
