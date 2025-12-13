use std::fmt::Debug;

use derive_new::new as New;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils::random_double,
    vec3::{Color, Vec3},
};

pub trait Material: Debug {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        return false;
    }
}

#[derive(Debug, Clone, Copy, New, Default)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo.clone();
        return true;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };

        Metal {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(ray_in.direction(), rec.normal).unit_vector()
            + (Vec3::random_unit_vector() * self.fuzz);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo.clone();
        return Vec3::dot(scattered.direction(), rec.normal) > 0.0;
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric {
            refraction_index: refraction_index,
        }
    }

    fn reflectance(cosin: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        return r0 + (1.0 - r0) * (1.0 - cosin).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction);
        return true;
    }
}
