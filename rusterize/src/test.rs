use crate::math::geometry::Triangle;
use crate::math::vectors::Vec3;
use crate::math::vectors::VectorMath;

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
    assert_eq!(a.norm(), b);
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
