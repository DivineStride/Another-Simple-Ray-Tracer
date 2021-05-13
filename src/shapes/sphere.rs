use crate::controls::hit::{Hit, HitRecord};
use crate::materials::scatter::Scatter;
use crate::rendering::rays::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Scatter>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return None }
        
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: ray.at(root),
            material: self.material.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}
