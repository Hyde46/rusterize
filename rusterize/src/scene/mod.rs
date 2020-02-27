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
    pub fn cornel_box(spp: u32) -> Self {
        let mut vec = Vec::new();

        // Floor
        vec.push(Triangle::new(
            Vec3::new(-150.0, 0.0, 0.0),
            Vec3::new(-150.0, 0.0, 300.0),
            Vec3::new(150.0, 0.0, 300.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(-150.0, 0.0, 0.0),
            Vec3::new(150.0, 0.0, 300.0),
            Vec3::new(150.0, 0.0, 0.0),
        ));

        // right hand side
        vec.push(Triangle::new(
            Vec3::new(150.0, 0.0, 300.0),
            Vec3::new(150.0, 300.0, 300.0),
            Vec3::new(150.0, 0.0, 0.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(150.0, 0.0, 0.0),
            Vec3::new(150.0, 300.0, 300.0),
            Vec3::new(150.0, 300.0, 0.0),
        ));

        // left hand side
        vec.push(Triangle::new(
            Vec3::new(-150.0, 0.0, 300.0),
            Vec3::new(-150.0, 0.0, 0.0),
            Vec3::new(-150.0, 300.0, 300.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(-150.0, 0.0, 0.0),
            Vec3::new(-150.0, 300.0, 0.0),
            Vec3::new(-150.0, 300.0, 300.0),
        ));

        // Ceiling
        vec.push(Triangle::new(
            Vec3::new(-150.0, 300.0, 0.0),
            Vec3::new(150.0, 300.0, 300.0),
            Vec3::new(-150.0, 300.0, 300.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(-150.0, 300.0, 0.0),
            Vec3::new(150.0, 300.0, 0.0),
            Vec3::new(150.0, 300.0, 300.0),
        ));

        // Back wall
        vec.push(Triangle::new(
            Vec3::new(-150.0, 0.0, 300.0),
            Vec3::new(-150.0, 300.0, 300.0),
            Vec3::new(150.0, 0.0, 300.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(150.0, 0.0, 300.0),
            Vec3::new(-150.0, 300.0, 300.0),
            Vec3::new(150.0, 300.0, 300.0),
        ));
        // Small Box in Scene
        // Front
        vec.push(Triangle::new(
            Vec3::new(-15.0, 0.0, 215.0),
            Vec3::new(-15.0, 30.0, 215.0),
            Vec3::new(15.0, 0.0, 215.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(15.0, 0.0, 215.0),
            Vec3::new(-15.0, 30.0, 215.0),
            Vec3::new(15.0, 30.0, 215.0),
        ));

        // Back wall
        vec.push(Triangle::new(
            Vec3::new(-15.0, 0.0, 230.0),
            Vec3::new(15.0, 0.0, 230.0),
            Vec3::new(-15.0, 30.0, 230.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(15.0, 0.0, 230.0),
            Vec3::new(15.0, 30.0, 230.0),
            Vec3::new(-15.0, 30.0, 230.0),
        ));

        // right hand side
        vec.push(Triangle::new(
            Vec3::new(15.0, 0.0, 230.0),
            Vec3::new(15.0, 0.0, 200.0),
            Vec3::new(15.0, 30.0, 230.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(15.0, 0.0, 200.0),
            Vec3::new(15.0, 30.0, 200.0),
            Vec3::new(15.0, 30.0, 230.0),
        ));

        // left hand side
        vec.push(Triangle::new(
            Vec3::new(-15.0, 0.0, 230.0),
            Vec3::new(-15.0, 0.0, 200.0),
            Vec3::new(-15.0, 30.0, 230.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(-15.0, 0.0, 200.0),
            Vec3::new(-15.0, 30.0, 200.0),
            Vec3::new(-15.0, 30.0, 230.0),
        ));

        // Ceiling
        vec.push(Triangle::new(
            Vec3::new(-15.0, 30.0, 200.0),
            Vec3::new(-15.0, 30.0, 230.0),
            Vec3::new(15.0, 30.0, 230.0),
        ));
        vec.push(Triangle::new(
            Vec3::new(-15.0, 30.0, 200.0),
            Vec3::new(15.0, 30.0, 230.0),
            Vec3::new(15.0, 30.0, 200.0),
        ));

        Scene {
            triangles: vec,
            samples_per_pixel: spp,
        }
    }

    pub fn single_triangle(spp: u32) -> Self {
        let mut vec = Vec::new();
        vec.push(Triangle::new(
            Vec3::new(-100.0, 0.0, 1.0),
            Vec3::new(100.0, 100.0, 150.0),
            Vec3::new(0.0, -100.0, 150.0),
        ));
        Scene {
            triangles: vec,
            samples_per_pixel: spp,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<IntersectionRecord> {
        let mut i_records: Vec<IntersectionRecord> = Vec::new();
        let mut min_dist = 1_000_000_f32;
        for t in &self.triangles {
            let mut temp_i_rec = IntersectionRecord::new();
            if t.intersects(ray, &mut temp_i_rec) {
                if temp_i_rec.distance > ray.max_dist || temp_i_rec.distance < ray.min_dist {
                    continue;
                }
                if temp_i_rec.distance < min_dist {
                    min_dist = temp_i_rec.distance;
                }
                i_records.push(temp_i_rec);
            }
        }
        for rec in i_records {
            if rec.distance == min_dist {
                return Some(rec.clone());
            }
        }
        None
    }
}
