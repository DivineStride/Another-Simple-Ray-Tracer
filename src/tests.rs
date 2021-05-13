use super::*;
use crate::vec3::Vec3;
use crate::R;
use rayon::prelude::*;
use std::f32::consts::PI;

#[test]
fn rayon_multithreading_from_iterator() {
    for _ in 0..5 {
        let color: Vec3 = (0..5)
            .into_par_iter()
            .map(|_index| Vec3::new(0.1, 0.1, 0.1))
            .sum();

        let against = Vec3::new(0.5, 0.5, 0.5);
        assert_eq!(against.x, color.x);
        assert_eq!(against.y, color.y);
        assert_eq!(against.z, color.z);
    }
}
