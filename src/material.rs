use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::{self, Ray};
use crate::utils::rand_vec3_in_unit_sphere;
use crate::{utils, vec3};

pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material: Sync + Send {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    base_color: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + utils::rand_vec3_unit();
        if vec3::is_near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        Some(Scatter {
            attenuation: self.base_color,
            ray: Ray::new(rec.p, scatter_direction),
        })
    }
}

impl Lambertian {
    pub fn new(base_color: Color) -> Self {
        Self {
            base_color: base_color,
        }
    }
}

pub struct Metal {
    base_color: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(base_color: Color, fuzz: f64) -> Self {
        Self {
            base_color: base_color,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = ray::reflect(r.direction().normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * rand_vec3_in_unit_sphere());

        if scattered.direction().dot(rec.normal) < 0.0 {
            return None;
        } else {
            return Some(Scatter {
                attenuation: self.base_color,
                ray: scattered,
            });
        }
    }
}

pub struct Dielectric {
    pub ir: f64, // index of refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r.direction().normalize();
        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let dir_out = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > utils::rand_f64()
        {
            ray::reflect(unit_direction, rec.normal)
        } else {
            ray::refract(r.direction().normalize(), rec.normal, refraction_ratio)
        };

        let scatter = Scatter {
            attenuation: Color::splat(1.0),
            ray: Ray::new(rec.p, dir_out),
        };
        Some(scatter)
    }
}
