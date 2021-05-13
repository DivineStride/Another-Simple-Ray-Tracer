use crate::degrees_to_radians;
use crate::rendering::rays::Ray;
use crate::vec3::{Point3, Vec3};
use rand::{thread_rng, Rng};

#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        v_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = degrees_to_radians(v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner: look_from - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            u,
            w,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Self::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }

    fn random_in_unit_disk() -> Vec3 {
        let mut rng = thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);
            if p.length_squared() > 1.0 {
                continue;
            } else {
                return p;
            }
        }
    }
}
