use crate::renderer::renderstructs::OrthogonalCamera;
use crate::scene::Scene;

pub type RgbaImage = im::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn render_scene(mut buffer: RgbaImage, _scene: &Scene, _cam: &OrthogonalCamera) -> RgbaImage {
    // Iterate over all pixels, sample each pixel and produce num of samples ray per pixel.
    // Call integrator per pixel based on ray
    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            //Sample pixel (x,y)
            //Generate ray with origin cam

            //Integrate over scene and add to cumulative L_i
            //Write L_i to buffer at (x,y)
            buffer.put_pixel(x, y, im::Rgba([(x % 600) as u8, (y % 600) as u8, 0, 255]));
        }
    }
    buffer
}
