use crate::math::vectors::Vec3;
use crate::math::vectors::VectorMath;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub min_dist: f32,
    pub max_dist: f32,
}

#[derive(Debug, PartialEq)]
pub struct IntersectionRecord {
    // Implements DataStructure holding information about
    // a point in 3D space where a ray may intersect with an other
    // object. Offers all necessary information to calculate
    // reflection properties
    pub normal: Vec3,
    pub distance: f32,
    pub hit_world: Vec3,
    pub hit_object: Vec3,
    pub hit: bool,
}

pub struct OrthogonalCamera {
    pub direction: Vec3,
    pub position: Vec3,
    pub up: Vec3,
    pub focal_length: f32,
}

// %%%%%%%%%%%%%%%%%%%%%%%
// %%%% struct impl  %%%%%
// %%%%%%%%%%%%%%%%%%%%%%%
impl IntersectionRecord {
    pub fn new() -> Self {
        IntersectionRecord {
            normal: Vec3::empty(),
            distance: 0.0,
            hit_world: Vec3::empty(),
            hit_object: Vec3::empty(),
            hit: false,
        }
    }
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, min_dist: f32, max_dist: f32) -> Self {
        Ray {
            origin,
            dir,
            min_dist,
            max_dist,
        }
    }
}

impl OrthogonalCamera {
    pub fn new(direction: Vec3, position: Vec3, up: Vec3, focal_length: f32) -> Self {
        OrthogonalCamera {
            direction,
            position,
            up,
            focal_length,
        }
    }
}
