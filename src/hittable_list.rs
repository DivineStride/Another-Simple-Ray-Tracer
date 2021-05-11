use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::rays::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Vec3 as Point3, Vec3 as Color};
use rand::{thread_rng, Rng};

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>, generate_random_scene: bool) -> Self {
        let mut hittables = Self { list };
        if generate_random_scene {
            hittables.random_scene();
        }

        hittables
    }

    fn random_scene(&mut self) {
        let mut rng = thread_rng();
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rng.gen::<f32>();
                let center = Point3::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let sphere_material = if choose_mat < 0.8 {
                        // Diffuse
                        let albedo = Color::random() * Color::random();
                        Material::Lambertian { albedo }
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = Color::random_within(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..=0.5);
                        Material::Metal { albedo, fuzz }
                    } else {
                        // Glass
                        Material::Dielectric {
                            refraction_index: 1.5,
                        }
                    };

                    self.list
                        .push(Box::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;
        for item in self.list.iter() {
            if let Some(rec) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}
