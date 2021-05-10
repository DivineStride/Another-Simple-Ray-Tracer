use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::vec3::{Vec3 as Point3, Vec3 as Color};
// use crate::R;

pub fn get_world() -> HittableList {
    // let material_left = Material::Lambertian {
    //     albedo: Color::new(0.0, 0.0, 1.0)
    // };
    // let material_right = Material::Lambertian {
    //     albedo: Color::new(1.0, 0.0,  0.0)
    // };
    //
    // let list: Vec<Box<dyn Hittable>> = vec![
    //     Box::new(Sphere {
    //         center: Point3::new(-R.cos(), 0.0, -1.0),
    //         radius: R.cos(),
    //         material: material_left
    //     }),
    //     Box::new(Sphere {
    //         center: Point3::new(R.cos(), 0.0, -1.0),
    //         radius: R.cos(),
    //         material: material_right
    //     })
    // ];

    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
        // refraction_index: 1.5,
    };
    let material_left = Material::Dielectric {
        refraction_index: 1.5,
    };
    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0012,
    };

    let list: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_ground,
        }),
        Box::new(Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: material_center,
        }),
        Box::new(Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left,
        }),
        Box::new(Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: -0.35,
            material: material_left,
        }),
        Box::new(Sphere {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right,
        }),
    ];

    HittableList::new(list)
}
