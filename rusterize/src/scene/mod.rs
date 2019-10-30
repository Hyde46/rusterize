use crate::math::geometry::Triangle;
use crate::math::vectors::Vec3;

pub struct Scene{
    pub triangles: Vec<Triangle>,
}

impl Scene {
    pub fn single_triangle() -> Self {
        let mut vec = Vec::new();
        vec.push(Triangle::new(Vec3::new(0.0,0.0,1.0), Vec3::new(2.0,2.0,1.0), Vec3::new(2.0,0.0,1.0)));
        Scene{
            triangles: vec, 
        }
    }
}
