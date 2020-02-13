mod math;
mod renderer;
mod scene;
mod utils;

#[macro_use]
extern crate approx;

extern crate image as im;
extern crate piston_window;
extern crate vecmath;

use piston_window::*;
use renderer::raytracer::render_scene;
use renderer::renderstructs::OrthogonalCamera;
use utils::RenderType;

use crate::math::vectors::Vec3;

use crate::scene::Scene;

const RENDER_TYPE: RenderType = RenderType::raytracer;

pub type RgbaImage = im::ImageBuffer<im::Rgba<u8>, Vec<u8>>;

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (600, 600);
    let mut window: PistonWindow = WindowSettings::new("Rusterize", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut draw = false;
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let scene = Scene::single_triangle();
    let mut cam = OrthogonalCamera::new(
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -5.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        width,
        height,
    );

    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &cam.film, &TextureSettings::new()).unwrap();

    let mut last_pos: Option<[f64; 2]> = None;

    //TODO: replace code with renderer based on RENDER_TYPE
    render_scene(&scene, &mut cam);
    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &cam.film).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);
                clear([1.0; 4], g);

                //canvas.put_pixel(index%600, index%600, im::Rgba([(index % 255) as u8, 0, 0, 255]));
                image(&texture, c.transform, g);
            });
        }
    }
}

#[cfg(test)]
mod test;

/*
if let Some(button) = e.press_args() {
    if button == Button::Mouse(MouseButton::Left) {
        draw = true;
    }
};
if let Some(button) = e.release_args() {
    if button == Button::Mouse(MouseButton::Left) {
        draw = false;
        last_pos = None
    }
};
if draw {
    if let Some(pos) = e.mouse_cursor_args() {
        let (x, y) = (pos[0] as f32, pos[1] as f32);

        if let Some(p) = last_pos {
            let (last_x, last_y) = (p[0] as f32, p[1] as f32);
            let distance = vec2_len(vec2_sub(p, pos)) as u32;

            for i in 0..distance {
                let diff_x = x - last_x;
                let diff_y = y - last_y;
                let delta = i as f32 / distance as f32;
                let new_x = (last_x + (diff_x * delta)) as u32;
                let new_y = (last_y + (diff_y * delta)) as u32;
                if new_x < width && new_y < height {
                    canvas.put_pixel(new_x, new_y, im::Rgba([0, 0, 0, 255]));
                };
            }
        };

        last_pos = Some(pos)
    };
}
*/
