use std::{fmt::Debug, rc::Rc};

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material: Debug {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        return false;
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = &rec.normal + &Vec3::random_unit_vector();
        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo.clone();
        return true;
    }
}

#[derive(Debug, Clone)]
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
        let reflected = &Vec3::reflect(ray_in.direction(), &rec.normal).unit_vector()
            + &(&Vec3::random_unit_vector() * self.fuzz);
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo.clone();
        return Vec3::dot(scattered.direction(), &rec.normal) > 0.0;
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric {
            refraction_index: refraction_index,
        }
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
        let cos_theta = unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, ri)
        };

        *scattered = Ray::new(&rec.p, &direction);
        return true;
    }
}
