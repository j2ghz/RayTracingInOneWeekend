use rand::{thread_rng, Rng};

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
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scattered> {
        let reflected = reflect(ray.direction().unit_vector(), hit.normal);
        let scattered = Ray::new(
            hit.p,
            reflected + Vec3d::random_in_unit_sphere() * self.fuzz,
        );
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

#[derive(Clone, Debug)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scattered> {
        let reflected = reflect(ray.direction(), hit.normal);
        let attenuation = Vec3d::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(hit.normal) > 0.0 {
            (
                -hit.normal,
                self.ref_idx,
                self.ref_idx * ray.direction().dot(hit.normal) / ray.direction().length(),
            )
        } else {
            (
                hit.normal,
                1.0 / &self.ref_idx,
                -ray.direction().dot(hit.normal) / ray.direction().length(),
            )
        };

        Some(Scattered {
            attenuation,
            ray: Ray::new(
                hit.p,
                if let Some(refracted) = refract(ray.direction(), outward_normal, ni_over_nt) {
                    let reflection_probability = schlick(cosine, self.ref_idx);
                    if reflection_probability < thread_rng().gen() {
                        reflected
                    } else {
                        refracted
                    }
                } else {
                    reflected
                },
            ),
        })
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(v: Vec3d, normal: Vec3d, ni_over_nt: f64) -> Option<Vec3d> {
    let uv = v.unit_vector();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = (uv - normal * dt) * ni_over_nt - normal * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}
