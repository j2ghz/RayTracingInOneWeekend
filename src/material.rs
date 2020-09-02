use crate::{geometry::HitRecord, ray::Ray, vec3d::Vec3d};

pub struct Scattered {
    pub ray: Ray,
    pub attenuation: Vec3d,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scattered>;
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vec3d,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scattered> {
        let target = hit.p + hit.normal + Vec3d::random_in_unit_sphere();
        let ray = Ray::new(hit.p, target - hit.p);
        Some(Scattered {
            ray,
            attenuation: self.albedo,
        })
    }
}

fn reflect(v: Vec3d, n: Vec3d) -> Vec3d {
    v - n * v.dot(n) * 2.0
}

#[derive(Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3d,
}
impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scattered> {
        let reflected = reflect(ray.direction().unit_vector(), hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        if scattered.direction().dot(hit.normal) > 0.0 {
            Some(Scattered {
                ray: scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
