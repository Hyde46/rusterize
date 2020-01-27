use crate::renderer::renderstructs::Camera;
use crate::scene::Scene;

pub type RgbaImage = im::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn render_scene<'a>(_buffer: &'a RgbaImage, _scene: &Scene, _cam: &Camera) {}
