use crate::math::vectors::Vec3;
use crate::math::vectors::VectorMath;

extern crate image as im;
extern crate rand;
use rand::prelude::*;
pub type RgbaImage = im::ImageBuffer<im::Rgba<u8>, Vec<u8>>;

trait RayGenerator {
    // General Camera trait
    // Every type of camera has to be able to get sampled for rays
    fn sample_pixel(&self, u: f32, v: f32) -> CameraSample;
    fn generate_ray(&self, camera_sample: CameraSample) -> Ray;
}

#[derive(Debug, PartialEq)]
pub struct CameraSample {
    // Sample in camera
    // Image_x image_y are sample on image Plane
    // lens_u lens_v samples on the lens of the camera
    pub image_x: f32,
    pub image_y: f32,
    pub lens_u: f32,
    pub lens_v: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    // Ray struct to model rays traversing in the scene
    pub origin: Vec3,
    pub dir: Vec3,
    pub min_dist: f32,
    pub max_dist: f32,
}

#[derive(Debug, PartialEq, Clone)]
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
    pub film_width: u32,
    pub film_height: u32,
    pub film: RgbaImage,
}

pub struct PerspectiveCamera {
    pub direction: Vec3,
    pub position: Vec3,
    pub up: Vec3,
    pub focal_length: f32,
    pub film_width: u32,
    pub film_height: u32,
    pub film: RgbaImage,
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
    pub fn new_image_plane(x: f32, y: f32) -> Self {
        CameraSample {
            image_x: x,
            image_y: y,
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
    pub fn new(
        direction: Vec3,
        position: Vec3,
        up: Vec3,
        focal_length: f32,
        film_width: u32,
        film_height: u32,
    ) -> Self {
        let film = RgbaImage::new(film_width, film_height);
        OrthogonalCamera {
            direction,
            position,
            up,
            focal_length,
            film_width,
            film_height,
            film,
        }
    }
    pub fn generate_ray(&self, camera_sample: CameraSample) -> Ray {
        //TODO: Generate ray based on camera sample
        Ray::new(
            self.position.clone()
                + Vec3::new(
                    camera_sample.image_x - 300_f32, //TODO: Replace with not hardcoded stuff
                    camera_sample.image_y - 300_f32,
                    0f32,
                ),
            self.direction.clone(),
            0_f32,
            10000_f32,
        )
    }
    pub fn sample_pixel(&self, u: f32, v: f32, rng: &mut ThreadRng) -> CameraSample {
        let x_rnd: f32 = rng.gen();
        let y_rnd: f32 = rng.gen();
        CameraSample::new_image_plane(u + x_rnd, v + y_rnd)
    }
}

impl PerspectiveCamera {
    pub fn new(
        direction: Vec3,
        position: Vec3,
        up: Vec3,
        focal_length: f32,
        film_width: u32,
        film_height: u32,
    ) -> Self {
        let film = RgbaImage::new(film_width, film_height);
        PerspectiveCamera {
            direction,
            position,
            up,
            focal_length,
            film_width,
            film_height,
            film,
        }
    }
    pub fn generate_ray(&self, camera_sample: CameraSample) -> Ray {
        let pixel_position = self.position.clone()
            + (self.direction.clone()).scale(self.focal_length)
            + Vec3::new(
                camera_sample.image_x - 300_f32,
                camera_sample.image_y - 300_f32,
                0f32,
            );
        Ray::new(
            self.position.clone(),
            (pixel_position - self.position.clone()).normalize(),
            0_f32,
            1000_f32,
        )
    }
    pub fn sample_pixel(&self, u: u32, v: u32, rng: &mut ThreadRng) -> CameraSample {
        let x_rnd: f32 = rng.gen();
        let y_rnd: f32 = rng.gen();
        CameraSample::new_image_plane(u as f32 + x_rnd, (self.film_height - v) as f32 - y_rnd)
    }
}
