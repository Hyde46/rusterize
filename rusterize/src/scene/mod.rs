use crate::math::geometry::Intersectable;
use crate::math::geometry::Triangle;
use crate::math::vectors::Vec3;
use crate::renderer::renderstructs::IntersectionRecord;
use crate::renderer::renderstructs::Ray;

pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub samples_per_pixel: u32,
}

impl Scene {
    pub fn single_triangle(spp: u32) -> Self {
        let mut vec = Vec::new();
        vec.push(Triangle::new(
            Vec3::new(-100.0, 0.0, 1.0),
            Vec3::new(0.0, 100.0, 1.0),
            Vec3::new(100.0, 0.0, 2.0),
        ));
        Scene {
            triangles: vec,
            samples_per_pixel: spp,
        }
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
