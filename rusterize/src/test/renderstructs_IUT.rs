use crate::math::vectors::Vec3;
use crate::renderer::renderstructs::Ray;

#[test]
fn clone_vec3() {
    let v1 = Vec3::new(0f32, 1f32, 2f32);
    assert_eq!(v1.clone(), Vec3::new(0f32, 1f32, 2f32));
}

#[test]
fn clone_ray() {
    let r1 = Ray::new(Vec3::empty(), Vec3::empty(), 0_f32, 10000_f32);
    assert_eq!(r1, r1.clone());
}
