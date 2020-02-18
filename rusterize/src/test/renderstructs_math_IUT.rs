use crate::math::geometry::Intersectable;
use crate::math::geometry::Triangle;

use crate::math::vectors::Vec3;

use crate::renderer::renderstructs::IntersectionRecord;
use crate::renderer::renderstructs::Ray;

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

    let mut i_rec = IntersectionRecord::new();
    assert!(t.intersects(&ray, &mut i_rec));

    let origin = Vec3::new(5.5, 2.2, 2.0);
    let dir = Vec3::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, dir, 10000.0, 100_000.0);
    assert!(!t.intersects(&ray, &mut i_rec));
}

#[test]
fn test_ray_miss_triangle_wrong_dir() {
    let t = Triangle {
        v1: Vec3::new(0.0, 0.0, 0.0),
        v2: Vec3::new(10.0, 0.0, 0.0),
        v3: Vec3::new(10.0, 10.0, 0.0),
    };
    let mut i_rec = IntersectionRecord::new();
    let origin = Vec3::new(-5.5, 2.2, 2.0);
    let dir = Vec3::new(0.0, 0.0, -1.0);
    let ray = Ray::new(origin, dir, 10000.0, 100_000.0);
    assert!(!t.intersects(&ray, &mut i_rec));
}

#[test]
fn test_ray_hit_triangle_not_in_triangle() {
    let t = Triangle {
        v1: Vec3::new(0.0, 0.0, 0.0),
        v2: Vec3::new(10.0, 0.0, 0.0),
        v3: Vec3::new(10.0, 10.0, 0.0),
    };
    let mut i_rec = IntersectionRecord::new();
    // Ray hits Plane in which triangle lies, but misses triangle
    let origin = Vec3::new(5.5, 2.2, 2.0);
    let dir = Vec3::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, dir, 10000.0, 100_000.0);
    assert!(!t.intersects(&ray, &mut i_rec));
}
