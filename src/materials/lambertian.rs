use crate::controls::hit::HitRecord;
use crate::materials::scatter::Scatter;
use crate::rendering::rays::Ray;
use crate::vec3::Vec3 as Color;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn with(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Color::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}
