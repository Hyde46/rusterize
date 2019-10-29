use crate::math::vectors::*;

const EPSILON: f32 = 0.00002_f32;
// %%%%%%%%%%%%%%%%%%%%%%%
// %%%%   Structs    %%%%%
// %%%%%%%%%%%%%%%%%%%%%%%
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
        let (mut uu, mut uv, mut vv, mut wu, mut wv, mut D) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        uu = u.dot(&u);
        uv = u.dot(&v);
        vv = v.dot(&v);
        w = i_rec.hit_world.sub(&self.v1);
        wu = w.dot(&u);
        wv = w.dot(&v);
        D = uv * uv - uu * vv;

        // get and test parametric coords
        let s = (uv * wv - vv * wu) / D;
        if s < 0.0 || s > 1.0 {
            // I is outside T
            return false;
        }
        let t = (uv * wu - uu * wv) / D;
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
