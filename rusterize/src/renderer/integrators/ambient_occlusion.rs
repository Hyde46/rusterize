use crate::math::vectors::Vec3;
use crate::math::vectors::VectorMath;
use crate::renderer::renderstructs::IntersectionRecord;
use crate::renderer::renderstructs::PerspectiveCamera;
use crate::renderer::renderstructs::Ray;
use crate::scene::Scene;

extern crate rand;
use rand::prelude::*;
use rand::rngs::ThreadRng;

pub fn integrate(scene: &Scene, cam: &PerspectiveCamera, ray: &Ray, rng: &mut ThreadRng) -> Vec3 {
    let ao_samples = 5;
    let ao_sample_fraction = 1.0 / ao_samples as f32;
    let mut L_i: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    let mut i_rec = IntersectionRecord::new();

    // First intersection in scene
    if let Some(i_rec) = scene.intersect(&ray) {
        // Generate <ao_samples> amount of samples at each intersection
        for _ in 0..ao_samples {
            let ao_dir = Vec3::new(rng.gen(), rng.gen(), rng.gen());
            let ao_ray = Ray::new(i_rec.hit_world.clone(), ao_dir, std::f32::EPSILON, 50.0_f32);

            let mut ao_i_rec = IntersectionRecord::new();
            if let Some(ao_i_rec) = scene.intersect(&ao_ray) {
                L_i = L_i.add(&Vec3::new(
                    -ao_sample_fraction,
                    -ao_sample_fraction,
                    -ao_sample_fraction,
                ));
            }
        }
    }
    L_i
}
