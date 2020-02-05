use crate::math::vectors::Vec3;

trait Camera {
    fn sample_pixel(&self, u: f32, v: f32) -> CameraSample;
    fn generate_ray(&self, camera_sample: CameraSample) -> Ray;
}

#[derive(Debug, PartialEq)]
pub struct CameraSample {
    pub image_x: f32,
    pub image_y: f32,
    pub lens_u: f32,
    pub lens_v: f32,
}

#[derive(Debug, PartialEq, Clone)]
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
impl CameraSample {
    pub fn new() -> Self {
        CameraSample {
            image_x: 0.0,
            image_y: 0.0,
            lens_u: 0.0,
            lens_v: 0.0,
        }
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

impl Camera for OrthogonalCamera {
    fn sample_pixel(&self, u: f32, v: f32) -> CameraSample {
        //TODO: Actually sample image plane by pixel (x,y) return world coordinates
        CameraSample::new()
    }

    fn generate_ray(&self, camera_sample: CameraSample) -> Ray {
        //TODO: Generate ray based on camera sample
        Ray::new(
            self.position.clone() + Vec3::new(camera_sample.image_x, camera_sample.image_y, 0f32),
            self.direction.clone(),
            0_f32,
            10000_f32,
        )
    }
}
