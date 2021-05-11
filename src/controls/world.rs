use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::vec3::{Vec3 as Point3, Vec3 as Color};
// use crate::R;

pub fn get_world(generate_random: bool) -> HittableList {
    let material_ground = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    let material_center = Material::Dielectric {
        refraction_index: 1.5,
    };
    let material_left = Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    let material_right = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    let list: Vec<Box<dyn Hittable>> = vec![
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

    HittableList::new(list, generate_random)
}
