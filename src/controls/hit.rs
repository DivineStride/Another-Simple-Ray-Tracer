use crate::materials::scatter::Scatter;
use crate::rendering::rays::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;


#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point3,
    pub material: Arc<dyn Scatter>,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) -> () {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        }
    }
}

pub trait Hit: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
