/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */



#[macro_use]
pub mod macros;
pub mod batch;
pub mod buffer;
pub mod font;
pub mod graphics;
pub mod renderer;
pub mod shape;
pub mod texture;
pub mod shader;
pub mod matrix;
pub mod vector;
pub mod graphics3d;
pub mod color;
pub mod framebuffer;

#[cfg(test)]
mod tests {
    use renderer::init_gl;

    use crate::{color, font::Font, framebuffer::FrameBuffer, renderer, shader::Shader, texture::{Image, TextureRegion, Texture}, vector::Vec3f};
    use std::time::SystemTime;

    use crate::graphics::*;
    use crate::graphics3d::*;
    use crate::matrix::*;
    use crate::color::*;

    use engine_core::{info_log};

    #[test]
    fn shader() {
        let mut win = engine_core::window::Window::new(10, 10, "").unwrap();
        init_gl(&mut win);

        let shader = Shader::from_file("res/shaders/graphics/shape.glsl");

        for name in shader.uniform_names() {
            info_log!("[{}]", name);
        }
    }

    #[test]
    fn text_width() {
        let mut win = engine_core::window::Window::new(600, 400, "Graphics").unwrap();
        win.make_current();
        renderer::init_gl(&mut win);

        let font = Font::new("res/fonts/arial.ttf", 64);
        info_log!("{}", font.text_width("F u k"));
    }

    #[test]
    fn text() {
        let mut win = engine_core::window::Window::new(600, 400, "Graphics").unwrap();
        win.make_current();
        renderer::init_gl(&mut win);

        let mut gfx = Graphics::new(&mut win);
        gfx.set_font(Font::new("res/fonts/arial.ttf", 100));

        let mut m = 0.0;

        let font_texture = TextureRegion::new_whole(&gfx.font().atlas);

        while !win.should_close() {

            m += 0.001;

            //gfx.clear_rgba8888(0x00_00_00_FF);

            if m >= 2.0 {
                m -= 2.0;
                gfx.clear(Color::from(0x00))
            }

            //gfx.set_scale(m, m);
            gfx.draw_string("Hmmm ja du det ska man undra sig", -0.5, -0.5);
            gfx.draw_string("yeet", 0.0, 0.0);

            gfx.texture(font_texture.clone());
            gfx.fill_rect(-1.0, -1.0, 2.0, 2.0);

            gfx.update();
            gfx.flush();
            win.poll_events();
            win.swap_buffers();
        }
    }

    #[test]
    fn cube() {
        let mut win = engine_core::window::Window::new(600, 400, "Graphics").unwrap();
        win.make_current();
        renderer::init_gl(&mut win);

        let mut gfx = Graphics3D::new(&mut win);
        //let font = Font::new("res/fonts/arial.ttf", 100);
        //let tex = TextureRegion::new_whole(&gfx.font().atlas);
        let tex = Texture::from_file("res/textures/test.png");
        let tex = TextureRegion::new_whole(&tex);
        gfx.texture(tex);
        // gfx.set_shape_shader(Shader::from_file("res/shaders/graphics/shape.glsl"));

        let mut m = 0.0;

        unsafe { renderer::std_renderer::enable(renderer::std_renderer::Capability::DepthTest); }

        // Projection Matrix
        let fov = 90.0;
        let aspect_ratio = gfx.frame_height() as f32 / gfx.frame_width() as f32;
        let fov_rad = 1.0 / (fov * 0.5 / 180.0 * std::f32::consts::PI).tan();

        let projection = Mat4x4f::projection(aspect_ratio, fov_rad, 100.0, 0.1);

        let mut time = SystemTime::now();
        let mut counter = 0;

        while !win.should_close() {
            counter += 1;

            if time.elapsed().unwrap().as_millis() >= 1000 {
                println!("fps: {}", counter);
                time = SystemTime::now();
                counter = 0;
            }

            let model = Mat4x4f::mult(&Mat4x4f::rotation_y(m), &Mat4x4f::rotation_x(1.0));
            let model = Mat4x4f::mult(&Mat4x4f::translation(0.0, m * 10.0, 2.0), &model);
            let mvp = Mat4x4f::mult(&projection, &model);
            m += 0.001;

            unsafe { renderer::std_renderer::clear(renderer::std_renderer::ClearTarget::Depth); }
            gfx.clear(Color::from(0x00_00_00_FF));

            gfx.set_color(Color::from((m%1.0, 1.0-m%1.0, 0.0, 1.0)));
            for i in 0..10 {
                for j in 0..10 {
                    for l in 0..10 {
                        gfx.fill_cube(1.0 * i as f32, 1.0 * j as f32, 1.0 * l as f32, 0.8, 0.8, 0.8,  &mvp);
                    }
                }
            }

            gfx.update();
            gfx.flush();
            win.poll_events();
            win.swap_buffers();
        }
    }

    #[test]
    fn gfx2d() {
        let mut win = engine_core::window::Window::new(600, 400, "Graphics").unwrap();
        win.make_current();
        renderer::init_gl(&mut win);

        let mut gfx = Graphics2D::new(&mut win);
        let mut i = 0.0;
        
        while !win.should_close() {
            i += 0.001;

            gfx.clear(Color::from(0x00_00_00_FF));

            let mat = Mat3x3f::translation(-0.5, 0.0);
            let mat = Mat3x3f::mult(&Mat3x3f::rotation(i), &mat);
            let mut corners = vec![
                Vec3f::new(0.0, 0.0, 1.0),
                Vec3f::new(1.0, 0.0, 1.0),
                Vec3f::new(0.0, 1.0, 1.0),
                Vec3f::new(1.0, 1.0, 1.0),
            ];
            
            gfx.set_color(color::RED);
            gfx.fill_rect(0.0, 0.0, 1.0, 1.0, &mat);
            
            for c in &mut corners {
                *c = Mat3x3f::mult_vec(&mat, *c);
                gfx.set_color(color::BLUE);
                gfx.fill_rect(c.x, c.y, 0.05, 0.05, &Mat3x3f::identity());
            }
            

            gfx.update();
            gfx.flush();
            win.poll_events();
            win.swap_buffers();
        }
    }

    #[test]
    fn framebuffer() {
        let mut win = engine_core::window::Window::new(600, 400, "Graphics").unwrap();
        win.make_current();
        renderer::init_gl(&mut win);

        let mut gfx = Graphics::new(&mut win);

        let mut fb = FrameBuffer::new(win.get_width() as u32, win.get_height() as u32);
        fb.bind();
        info_log!("d");
        //gfx.clear(color::ORANGE);
        //&gfx.texture(TextureRegion::new_invalid());
        gfx.set_color(color::BLUE);
        gfx.fill_ellipse(-1.0, -1.0, 1.0, 1.0);
        //gfx.fill_rect(-1.0, -1.0, 0.5, 1.0);
        gfx.flush();

        Image::from_framebuffer(&fb).to_file("res/textures/kuk.png");

        let mut i = 0;
        FrameBuffer::un_bind();
        while !win.should_close() {
            
            //fb.bind();
            gfx.texture(TextureRegion::new_invalid());
            gfx.set_color(color::WHITE);
            gfx.fill_rect(-1.0, -1.0, 0.5, 0.5);
            gfx.flush();
            Image::new(600, 400, FrameBuffer::get_pixels_standard_frame_buffer(0, 0, 600, 400)).to_file(&format!("res/textures/kuk{}.png", i));
            FrameBuffer::un_bind();
            i += 1;


            //gfx.texture(TextureRegion::new_whole(fb.texture()));

            /*gfx.texture(TextureRegion::new_invalid());
            gfx.set_color(color::WHITE);
            gfx.fill_rect(0.0, 0.0, 1.0, 1.0);*/

            gfx.update();
            gfx.flush();
            win.poll_events();
            win.swap_buffers();
        }
    }
}