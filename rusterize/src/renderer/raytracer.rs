use crate::renderer::renderstructs::IntersectionRecord;
use crate::renderer::renderstructs::OrthogonalCamera;
use crate::renderer::renderstructs::PerspectiveCamera;
use crate::scene::Scene;

pub type RgbaImage = im::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn render_scene(scene: &Scene, cam: &mut PerspectiveCamera) {
    let mut buffer = RgbaImage::new(cam.film_width, cam.film_height);
    // Iterate over all pixels, sample each pixel and produce num of samples ray per pixel.
    // Call integrator per pixel based on ray
    for x in 0..cam.film_width {
        for y in 0..cam.film_height {
            //Sample pixel (x,y)
            //Generate ray with origin cam
            let camera_sample = cam.sample_pixel(x as f32, y as f32);
            let ray = cam.generate_ray(camera_sample);
            let i_rec = IntersectionRecord::new();

            if scene.intersect(&ray, &i_rec) {
                buffer.put_pixel(x, y, im::Rgba([255 as u8, 255 as u8, 255, 255]));
            } else {
                buffer.put_pixel(x, y, im::Rgba([0 as u8, 0 as u8, 0, 255]));
            }
            //Integrate over scene and add to cumulative L_i
            //Write L_i to buffer at (x,y)
        }
    }
    println!("Done rendering!");
    cam.film = buffer;
    cam.film
        .save_with_format("./result", im::ImageFormat::PNG)
        .unwrap();
}
