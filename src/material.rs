use std::{fmt::Debug, rc::Rc};

use derive_new::new as New;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{self, SolidColor, Texture},
    utils::random_double,
    vec3::{Color, Point3, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        return false;
    }

    fn emitted(&self, u : f64, v : f64, point : Point3) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
}

#[derive(Clone, New)]
pub struct Lambertian {
    pub texture: Rc<dyn Texture>,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn from_color(albedo: Color) -> Self {
        Self::new(Rc::new(SolidColor::new(albedo)))
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
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction, ray_in.time);
        *attenuation = self.texture.value(rec.u, rec.v, rec.p);
        return true;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz.min(1.0),
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
        let reflected = Vec3::reflect(&ray_in.dir, rec.normal).unit_vector()
            + (Vec3::random_unit_vector() * self.fuzz);
        *scattered = Ray::new(rec.p, reflected, ray_in.time);
        *attenuation = self.albedo;
        return Vec3::dot(&scattered.dir, rec.normal) > 0.0;
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    pub refraction_index: f64,
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric {
            refraction_index: refraction_index,
        }
    }

    #[allow(dead_code)]
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

        let unit_direction = ray_in.dir.unit_vector();
        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction, ray_in.time);
        return true;
    }
}

#[derive(Clone, New)]
pub struct DiffuseLight {
    pub texture: Rc<dyn Texture>,
}

impl DiffuseLight {
    #[allow(dead_code)]
    pub fn from_color(albedo: Color) -> Self {
        Self::new(Rc::new(SolidColor::new(albedo)))
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u : f64, v : f64, point : Point3) -> Color {
        self.texture.value(u, v, point)
    }
}
