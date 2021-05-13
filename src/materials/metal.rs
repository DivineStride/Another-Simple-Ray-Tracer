use crate::controls::hit::HitRecord;
use crate::materials::scatter::Scatter;
use crate::rendering::rays::Ray;
use crate::vec3::{Vec3 as Color, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn with(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz > 1.0 { 1.0 } else { fuzz },
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = &ray_in.direction.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.p,
            *reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
