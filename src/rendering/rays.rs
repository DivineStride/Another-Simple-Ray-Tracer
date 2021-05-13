use crate::controls::hit::Hit;
use crate::controls::world::World;
use crate::vec3::{Color, Point3, Vec3};

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
    fn ray_color(ray: &Ray, world: &World, depth: u32) -> Self;
}

impl RayColor for Vec3 {
    fn ray_color(ray: &Ray, world: &World, depth: u32) -> Self {
        if let Some(rec) = world.hit(&ray, 0.001, f32::INFINITY) {
            match rec.material.scatter(ray, &rec) {
                Some((attenuation, scattered)) if depth > 0 => {
                    attenuation * Self::ray_color(&scattered, world, depth - 1)
                }
                _ => Color::default(),
            }
        } else {
            let unit_direction = ray.direction.unit_vector();
            let terp = 0.5 * (unit_direction.y + 1.0);
        
            (1.0 - terp) * Color::new(1.0, 1.0, 1.0) + terp * Color::new(0.5, 0.7, 1.0)
        }
    }
}
