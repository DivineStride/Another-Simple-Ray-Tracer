use rand::{thread_rng, Rng};

use crate::controls::hit::{Hit, HitRecord};
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::rendering::rays::Ray;
use crate::shapes::sphere::Sphere;
use crate::vec3::{Color, Point3};
use std::sync::Arc;

pub struct World(Vec<Box<dyn Hit>>);

impl Hit for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;
        for objects in &self.0 {
            if let Some(rec) = objects.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}

impl World {
    pub fn new(list: Vec<Box<dyn Hit>>, generate_random_scene: bool) -> Self {
        let mut hittables = Self(list);
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
                    if choose_mat < 0.8 {
                        // Diffuse
                        let albedo = Color::random() * Color::random();
                        self.0.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Lambertian::with(albedo)),
                        )));
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = Color::random_within(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..=0.5);
                        self.0.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Metal::with(albedo, fuzz)),
                        )));
                    } else {
                        // Glass
                        self.0.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Dielectric::with(1.5)),
                        )));
                    }
                }
            }
        }
    }

    pub fn get_world(generate_random: bool) -> Self {
        let material_ground = Arc::new(Lambertian::with(Color::new(0.5, 0.5, 0.5)));
        let material_center = Arc::new(Dielectric::with(1.5));
        let material_left = Arc::new(Lambertian::with(Color::new(0.4, 0.2, 0.1)));
        let material_right = Arc::new(Metal::with(Color::new(0.7, 0.6, 0.5), 0.0));

        let list: Vec<Box<dyn Hit>> = vec![
            Box::new(Sphere {
                center: Point3::new(0.0, -1000.0, -1.0),
                radius: 1000.0,
                material: material_ground,
            }),
            Box::new(Sphere {
                center: Point3::new(0.0, 1.0, 0.0),
                radius: 1.0,
                material: material_center,
            }),
            Box::new(Sphere {
                center: Point3::new(-4.0, 1.0, 0.0),
                radius: 1.0,
                material: material_left,
            }),
            Box::new(Sphere {
                center: Point3::new(4.0, 1.0, 0.0),
                radius: 1.0,
                material: material_right,
            }),
        ];

        Self::new(list, generate_random)
    }
}
