use crate::hittable::HitRecord;
use crate::rays::Ray;
use crate::vec3::{Vec3 as Color, Vec3};
use rand::random;
use std::ops::Neg;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { refraction_index: f32 },
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Color::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal
                }
                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = *albedo;

                true
            }
            Self::Metal { albedo, mut fuzz } => {
                let reflected = Self::reflect(&ray_in.direction.unit_vector(), &rec.normal);
                if fuzz > 1.0 {
                    fuzz = 1.0
                }
                *scattered = Ray::new(rec.p, reflected + fuzz * Vec3::random_in_unit_sphere());
                *attenuation = *albedo;

                scattered.direction.dot(rec.normal) > 0.0
            }
            Self::Dielectric { refraction_index } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face {
                    1.0 / *refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = ray_in.direction.unit_vector();
                let cos_theta = unit_direction.neg().dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let direction = if refraction_ratio * sin_theta > 1.0
                    || Self::reflectance(cos_theta, refraction_ratio) > random()
                {
                    Self::reflect(&unit_direction, &rec.normal)
                } else {
                    Self::refract(&unit_direction, &rec.normal, refraction_ratio)
                };

                *scattered = Ray::new(rec.p, direction);

                true
            }
        }
    }

    pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(*normal) * *normal
    }

    pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powf(2.0);

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
