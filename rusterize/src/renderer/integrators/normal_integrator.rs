use crate::math::vectors::Vec3;
use crate::math::vectors::VectorMath;
use crate::renderer::renderstructs::IntersectionRecord;
use crate::renderer::renderstructs::PerspectiveCamera;
use crate::renderer::renderstructs::Ray;
use crate::scene::Scene;

extern crate rand as rnd;
use rnd::rngs::ThreadRng;

pub fn integrate(scene: &Scene, cam: &PerspectiveCamera, ray: &Ray, rng: &mut ThreadRng) -> Vec3 {
    let mut L_i: Vec3 = Vec3::empty();

    let mut i_rec = IntersectionRecord::new();

    if let Some(i_rec) = scene.intersect(&ray) {
        L_i = L_i.add(&i_rec.normal);
    }

    L_i
}
