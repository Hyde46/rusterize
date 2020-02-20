use crate::math::vectors::Vec3;
use crate::math::vectors::VectorMath;
use crate::renderer::integrators;
use crate::renderer::renderstructs::IntersectionRecord;
use crate::renderer::renderstructs::PerspectiveCamera;
use crate::scene::Scene;

extern crate rand;

pub type RgbaImage = im::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn render_scene(scene: &Scene, cam: &mut PerspectiveCamera) {
    let mut rng = rand::thread_rng();
    let mut buffer = RgbaImage::new(cam.film_width, cam.film_height);
    // Iterate over all pixels, sample each pixel and produce num of samples ray per pixel.
    // Call integrator per pixel based on raycam
    for x in 0..cam.film_width {
        for y in 0..cam.film_height {
            let mut pixel_buffer: Vec<Vec3> = Vec::new();
            //Start with dark pixel
            pixel_buffer.push(Vec3::new(0.0, 0.0, 0.0));

            for _ in 0..scene.samples_per_pixel {
                //Sample pixel (x,y)
                //Generate ray with origin cam
                let camera_sample = cam.sample_pixel(x, y, &mut rng);
                let ray = cam.generate_ray(camera_sample);

                let L_i = integrators::depth_integrator(&scene, &cam, &ray, &mut rng);

                pixel_buffer.push(L_i);
                let acc_pixel_buffer: Vec3 = pixel_buffer.iter().sum();
                let normalized_pixel_buffer =
                    acc_pixel_buffer.scale(255.0 / scene.samples_per_pixel as f32);
                buffer.put_pixel(
                    x,
                    y,
                    im::Rgba([
                        normalized_pixel_buffer.x as u8,
                        normalized_pixel_buffer.y as u8,
                        normalized_pixel_buffer.z as u8,
                        255,
                    ]),
                );
            }
        }
    }
    println!("Done rendering!");
    cam.film = buffer;
    cam.film
        .save_with_format("./result.png", im::ImageFormat::PNG)
        .unwrap();
}
