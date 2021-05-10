use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::vec3::{Vec3 as Point3, Vec3 as Color, Vec3};

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
    fn ray_color(ray: &Ray, world: &HittableList, mut depth: u32) -> Self {
        // Setting the Bounce Limit
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        } else {
            depth -= 1;
        }

        if let Some(rec) = world.hit(&ray, 0.0001, f32::INFINITY) {
            let mut scattered = Ray::new(Vec3::default(), Vec3::default());
            let mut attenuation = Color::default();
            if rec
                .material
                .scatter(ray, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(&scattered, world, depth);
            }
            return attenuation;
            // let target = rec.p + rec.normal + Vec3::random_unit_vector();
            // let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);
            // let origin = rec.p;
            // let direction = target - rec.p;
            // return 0.5 * Vec3::ray_color(&Ray::new(origin, direction), &world, depth);
        }

        let unit_direction = ray.direction.unit_vector();
        let terp = 0.5 * (unit_direction.y + 1.0);

        (1.0 - terp) * Color::new(1.0, 1.0, 1.0) + terp * Color::new(0.5, 0.7, 1.0)
    }
}
