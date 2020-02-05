use crate::math::geometry::Intersectable;
use crate::math::geometry::Triangle;

use crate::math::vectors::Vec3;
use crate::math::vectors::VectorMath;

use crate::renderer::renderstructs::IntersectionRecord;
use crate::renderer::renderstructs::Ray;

#[test]
fn test_vector_math_add() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(1.5, 2.0, -10.0);
    let expected = Vec3::new(2.5, 2.0, -5.0);
    assert_eq!(a.add(&b), expected);
}

#[test]
fn test_vector_math_sub() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(1.5, 2.0, -10.0);
    let expected = Vec3::new(-0.5, -2.0, 15.0);
    assert_eq!(a - b, expected);
}

#[test]
fn test_vector_math_mul() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(1.5, 2.0, -10.0);
    let expected = Vec3::new(1.5, 0.0, -50.0);
    assert_eq!(a * b, expected);
}

#[test]
fn test_vector_math_div() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(2.0, 2.0, 10.0);
    let expected = Vec3::new(0.5, 0.0, 0.5);
    assert_eq!(a / b, expected);
}

#[test]
fn test_vector_math_div_by_zero() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(2.0, 2.0, 0.0);
    let result = std::panic::catch_unwind(|| a / b);
    assert!(result.is_err());
}

#[test]
fn test_vector_math_cross() {
    let a = Vec3::new(1.0, 2.0, 5.0);
    let b = Vec3::new(5.0, 1.0, 3.0);
    let expected = Vec3::new(1.0, 22.0, -9.0);
    assert_eq!(a.cross(&b), expected);
}

#[test]
fn test_vector_math_scale() {
    let a = Vec3::new(1.0, 2.0, 5.0);
    let factor = 10.5;
    let expected = Vec3::new(10.5, 21.0, 52.5);
    assert_eq!(a.scale(factor), expected);
}

#[test]
fn test_vector_math_dot() {
    let a = Vec3::new(3.0, 2.2, 8.0);
    let b = Vec3::new(5.0, 1.0, 3.0);
    let expected: f32 = 41.2;
    assert!(ulps_eq!(a.dot(&b), expected, max_ulps = 2));

    let a = Vec3::new(1.0, 1.0, 0.0);
    let b = Vec3::new(0.0, 0.0, 1.0);
    let expected: f32 = 0.;
    assert!(ulps_eq!(a.dot(&b), expected, max_ulps = 2));

    let a = Vec3::new(1.0, 1.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    let expected: f32 = 1.;
    assert!(ulps_eq!(a.dot(&b), expected, max_ulps = 2));
}

#[test]
fn test_vector_math_length() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let expected: f32 = 1.;
    assert!(ulps_eq!(a.length(), expected, max_ulps = 2));

    let a = Vec3::new(1.0, 1.0, 0.0);
    let expected: f32 = 2_f32.sqrt();
    assert!(ulps_eq!(a.length(), expected, max_ulps = 2));
}

#[test]
fn test_vector_math_norm() {
    let a = Vec3::new(1.0, 1.0, 0.0);
    let b = Vec3::new(1_f32 / 2_f32.sqrt(), 1_f32 / 2_f32.sqrt(), 0_f32);
    assert_eq!(a.normalize(), b);
}

#[test]
fn test_std_trait_add() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(1.5, 2.0, -10.0);
    let expected = Vec3::new(2.5, 2.0, -5.0);
    assert_eq!(a + b, expected);
}

#[test]
fn test_std_trait_sub() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(1.5, 2.0, -10.0);
    let expected = Vec3::new(-0.5, -2.0, 15.0);
    assert_eq!(a - b, expected);
}

#[test]
fn test_std_trait_mul() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(1.5, 2.0, -10.0);
    let expected = Vec3::new(1.5, 0.0, -50.0);
    assert_eq!(a * b, expected);
}

#[test]
fn test_std_trait_div() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(2.0, 2.0, 10.0);
    let expected = Vec3::new(0.5, 0.0, 0.5);
    assert_eq!(a / b, expected);
}

#[test]
fn test_std_trait_div_by_zero() {
    let a = Vec3::new(1.0, 0.0, 5.0);
    let b = Vec3::new(2.0, 2.0, 0.0);
    let result = std::panic::catch_unwind(|| a / b);
    assert!(result.is_err());
}

#[test]
fn test_triangle_normal() {
    let t = Triangle {
        v1: Vec3::empty(),
        v2: Vec3::new(1.0, 0.0, 0.0),
        v3: Vec3::new(1.0, 1.0, 0.0),
    };
    let expected_normal = Vec3::new(0.0, 0.0, 1.0);
    let result = t.normal();
    assert_eq!(expected_normal, result);
}

#[test]
fn test_ray_hit_triangle() {
    let t = Triangle {
        v1: Vec3::new(0.0, 0.0, 0.0),
        v2: Vec3::new(10.0, 0.0, 0.0),
        v3: Vec3::new(10.0, 10.0, 0.0),
    };
    let origin = Vec3::new(5.5, 2.2, 2.0);
    let dir = Vec3::new(0.0, 0.0, -1.0);
    let ray = Ray::new(origin, dir, 10000.0, 100_000.0);

    let i_rec = IntersectionRecord::new();
    assert!(t.intersects(&ray, &i_rec));

    let origin = Vec3::new(5.5, 2.2, 2.0);
    let dir = Vec3::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, dir, 10000.0, 100_000.0);
    assert!(!t.intersects(&ray, &i_rec));
}

#[test]
fn test_ray_miss_triangle_wrong_dir() {
    let t = Triangle {
        v1: Vec3::new(0.0, 0.0, 0.0),
        v2: Vec3::new(10.0, 0.0, 0.0),
        v3: Vec3::new(10.0, 10.0, 0.0),
    };
    let i_rec = IntersectionRecord::new();
    let origin = Vec3::new(-5.5, 2.2, 2.0);
    let dir = Vec3::new(0.0, 0.0, -1.0);
    let ray = Ray::new(origin, dir, 10000.0, 100_000.0);
    assert!(!t.intersects(&ray, &i_rec));
}

#[test]
fn test_ray_hit_triangle_not_in_triangle() {
    let t = Triangle {
        v1: Vec3::new(0.0, 0.0, 0.0),
        v2: Vec3::new(10.0, 0.0, 0.0),
        v3: Vec3::new(10.0, 10.0, 0.0),
    };
    let i_rec = IntersectionRecord::new();
    // Ray hits Plane in which triangle lies, but misses triangle
    let origin = Vec3::new(5.5, 2.2, 2.0);
    let dir = Vec3::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, dir, 10000.0, 100_000.0);
    assert!(!t.intersects(&ray, &i_rec));
}

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

/*
#[test]
fn generate_orthographic_ray() {
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let dir = Vec3::new(0.0, 0.0, 1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let cam = OrthogonalCamera::new(origin, dir, up, 1.0_f32);
}
*/
