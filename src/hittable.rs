use crate::material::Material;
use crate::rays::Ray;
use crate::vec3::{Vec3 as Point3, Vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point3,
    pub material: Material,
    pub normal: Vec3,
    pub front_face: bool,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
