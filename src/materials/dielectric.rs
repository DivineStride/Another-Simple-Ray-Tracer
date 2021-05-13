use crate::controls::hit::HitRecord;
use crate::materials::scatter::Scatter;
use crate::rendering::rays::Ray;
use crate::vec3::{Vec3, Vec3 as Color};
use rand::random;
use std::ops::Neg;

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn with(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    pub fn reflectance(&self, cosine: f32) -> f32 {
        let r0 = ((1.0 - self.refraction_index) / (1.0 + self.refraction_index)).powf(2.0);

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }

    pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = uv.neg().dot(*normal).min(1.0);
        let ray_out_perp = etai_over_etat * (*uv + cos_theta * *normal);
        let ray_out_parallel = (1.0 - ray_out_perp.length_squared()).abs().sqrt().neg() * *normal;

        // Long form of the equation
        // etai_over_etat * (*uv - *normal * uv.dot(*normal))
        //     - *normal
        //         * (1.0
        //             - etai_over_etat * etai_over_etat * (1.0 - uv.dot(*normal) * uv.dot(*normal)))
        //         .sqrt()
        ray_out_parallel + ray_out_perp
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = unit_direction.neg().dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction =
            if refraction_ratio * sin_theta > 1.0 || self.reflectance(cos_theta) > random() {
                unit_direction.reflect(&rec.normal)
            } else {
                Self::refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
