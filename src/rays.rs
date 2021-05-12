use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::vec3::{Vec3 as Point3, Vec3 as Color, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}

pub trait RayColor {
    fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Self;
}

impl RayColor for Vec3 {
    fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Self {
        if let Some(rec) = world.hit(&ray, 0.001, f32::INFINITY) {
            let mut scattered = Ray::new(Vec3::default(), Vec3::default());
            let mut attenuation = Color::default();
            // Setting the Bounce Limit
            if depth > 0 && rec
                .material
                .scatter(ray, &rec, &mut attenuation, &mut scattered) {
        
                attenuation * Self::ray_color(&scattered, world, depth - 1)
            } else {
                attenuation
            }
        } else {
            let unit_direction = ray.direction.unit_vector();
            let terp = 0.5 * (unit_direction.y + 1.0);
        
            (1.0 - terp) * Color::new(1.0, 1.0, 1.0) + terp * Color::new(0.5, 0.7, 1.0)
        }
    }
}
