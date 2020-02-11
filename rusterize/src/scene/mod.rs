use crate::math::geometry::Intersectable;
use crate::math::geometry::Triangle;
use crate::math::vectors::Vec3;
use crate::renderer::renderstructs::IntersectionRecord;
use crate::renderer::renderstructs::Ray;

pub struct Scene {
    pub triangles: Vec<Triangle>,
}

impl Scene {
    pub fn single_triangle() -> Self {
        let mut vec = Vec::new();
        vec.push(Triangle::new(
            Vec3::new(-100.0, 0.0, 1.0),
            Vec3::new(0.0, 100.0, 1.0),
            Vec3::new(100.0, 0.0, 1.0),
        ));
        Scene { triangles: vec }
    }

    pub fn intersect(&self, ray: &Ray, i_rec: &IntersectionRecord) -> bool {
        for t in &self.triangles {
            if t.intersects(ray, i_rec) {
                return true;
            }
        }
        false
    }
}
