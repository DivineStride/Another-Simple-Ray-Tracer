use crate::controls::hit::HitRecord;
use crate::rendering::rays::Ray;
use crate::vec3::Vec3 as Color;

pub trait Scatter: Sync + Send {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

impl std::fmt::Debug for dyn Scatter {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Scatter {{ from: {:?} }}", self)?;
        Ok(())
    }
}
