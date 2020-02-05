use crate::math::vectors::*;

use crate::renderer::renderstructs::IntersectionRecord;
use crate::renderer::renderstructs::Ray;

const EPSILON: f32 = 0.00002_f32;

// %%%%%%%%%%%%%%%%%%%%%%%
// %%%%   Structs    %%%%%
// %%%%%%%%%%%%%%%%%%%%%%%
#[derive(Debug, PartialEq, Clone)]
pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
}

// %%%%%%%%%%%%%%%%%%%%%%%
// %%%%   traits     %%%%%
// %%%%%%%%%%%%%%%%%%%%%%%
pub trait Intersectable {
    fn intersects(&self, ray: &Ray, i_rec: &IntersectionRecord) -> bool;
}
// %%%%%%%%%%%%%%%%%%%%%%%
// %%%% trait impl   %%%%%
// %%%%%%%%%%%%%%%%%%%%%%%
impl Intersectable for Triangle {
    fn intersects(&self, ray: &Ray, i_rec: &IntersectionRecord) -> bool {
        // Calculates an intersection between a ray and a triangle
        // Fills out IntersectionRecord if an intersection takes place
        // Returns true if ray intersects with triangle and false if it does not
        let mut i_rec = IntersectionRecord::new();

        let (mut u, mut v, mut n) = (Vec3::empty(), Vec3::empty(), Vec3::empty());
        let (mut dir, mut w0, mut w) = (Vec3::empty(), Vec3::empty(), Vec3::empty());
        let (mut r, mut a, mut b) = (0.0, 0.0, 0.0);

        // Get triangle edge vectors and plane normal
        u = self.v2.sub(&self.v1);
        v = self.v3.sub(&self.v1);
        n = u.cross(&v);
        // TODO: check if vector is 0 -> triangle is degenerate

        w0 = ray.origin.sub(&self.v1);
        a = -n.dot(&w0);
        b = n.dot(&ray.dir);
        if b.abs() < EPSILON {
            // ray is parallel to triangle plane
            if a == 0.0 {
                // ray lies in triangle plane
                return false;
            } /* else {
                  // ray disjoint from plane
                  return false;
              }*/
        }

        // get intersect point of ray with triangle lane
        r = a / b;
        if r < 0.0 {
            // ray goes away from triangle
            //no intersect
            return false;
        }
        i_rec.hit_world = ray.origin.add(&ray.dir.scale(r));
        i_rec.distance = i_rec.hit_world.sub(&ray.origin).length();
        i_rec.normal = Vec3::new(n.x, n.y, n.z);

        //is point in triangle
        let uu = u.dot(&u);
        let uv = u.dot(&v);
        let vv = v.dot(&v);
        w = i_rec.hit_world.sub(&self.v1);
        let wu = w.dot(&u);
        let wv = w.dot(&v);
        let d = uv * uv - uu * vv;

        // get and test parametric coords
        let s = (uv * wv - vv * wu) / d;
        if s < 0.0 || s > 1.0 {
            // I is outside T
            return false;
        }
        let t = (uv * wu - uu * wv) / d;
        if t < 0.0 || (s + t) > 1.0 {
            // I is outside T
            return false;
        }
        true
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%
// %%%% struct impl  %%%%%
// %%%%%%%%%%%%%%%%%%%%%%%
impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Triangle { v1, v2, v3 }
    }
    pub fn normal(&self) -> Vec3 {
        let a: Vec3 = self.v2.sub(&self.v1);
        let b: Vec3 = self.v3.sub(&self.v1);
        a.cross(&b).normalize()
    }
}
