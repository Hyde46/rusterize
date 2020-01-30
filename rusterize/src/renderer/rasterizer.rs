use crate::renderer::renderstructs::OrthogonalCamera;
use crate::scene::Scene;

pub type RgbaImage = im::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn rasterize_scene(
    mut buffer: RgbaImage,
    _scene: &Scene,
    _cam: &OrthogonalCamera,
) -> RgbaImage {
    buffer
}
