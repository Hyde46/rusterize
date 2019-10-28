use crate::math::vectors::*;

#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
}

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub min_dist: f32,
    pub max_dist: f32,
}

#[derive(Debug, PartialEq)]
pub struct IntersectionRecord {
    pub normal: Vec3,
    pub distance: f32,
    pub hit_world: Vec3,
    pub hit_object: Vec3,
    pub hit: bool,
}

pub trait Intersectable {
    fn intersects(&self, ray: &Ray, i_rec: &IntersectionRecord) -> bool;
}

impl Intersectable for Triangle {
    fn intersects(&self, ray: &Ray, i_rec: &IntersectionRecord) -> bool {
        let mut i_rec = IntersectionRecord::new();

        false
    }
}

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

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Triangle { v1, v2, v3 }
    }
    pub fn normal(&self) -> Vec3 {
        let a: Vec3 = self.v2.sub(&self.v1);
        let b: Vec3 = self.v3.sub(&self.v1);
        a.cross(&b).norm()
    }
}
