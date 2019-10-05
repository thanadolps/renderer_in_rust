use nalgebra::{Unit, Vector3};

use assert_approx_eq::assert_approx_eq;

pub fn calculate_reflect_ray(incoming_ray: &Unit<Vector3<f32>>, normal: &Unit<Vector3<f32>>)
    -> Unit<Vector3<f32>> {
    // http://paulbourke.net/geometry/reflected/
    let ri = incoming_ray.into_inner();
    let ray = ri - 2.0 * ri.dot(normal.as_ref()) * normal.into_inner();
    debug_normalize(ray)
}

pub fn debug_normalize(v: Vector3<f32>) -> Unit<Vector3<f32>> {
    if cfg!(debug_assertions) {
        let (unit_vec, magnitude) = Unit::new_and_get(v);
        assert_approx_eq!(magnitude, 1.0, 1e-6);
        unit_vec
    }
    else {
        dbg!(v);
        Unit::new_unchecked(v)
    }
}