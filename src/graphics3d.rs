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

use crate::renderer::graphics_renderer::{ShapeBatchRenderer, SpriteBatchRenderer};
use crate::renderer;
use crate::shader::Shader;
use crate::buffer::VertexBufferLayout;
use crate::texture;
use crate::font;
use crate::matrix;
use crate::color::*;

use engine_core::window;

#[derive(Copy, Clone, PartialEq)]
pub enum LastDraw {
    None,
    Plane,
    Line,
    Sphere,
    Cube,
    SpritePlane,
    SpriteLine,
    SpriteSphere,
    SpriteCube,
}

struct DrawingInformation {
    color:         Color,
    translation:        (f32, f32, f32),
    scale:              (f32, f32, f32),
    line_width:         f32,
    sphere_detail:     u32,
}

impl DrawingInformation {
    pub fn new() -> DrawingInformation {
        DrawingInformation {
            color: Color::from(0xFFFFFFFFu32),
            translation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            line_width: 0.01,
            sphere_detail: 100,
        }
    }
}

pub struct Graphics3D {
    has_texture:    bool,
    texture:        texture::TextureRegion, 
    font:           font::Font,
    
    shape_ren:      renderer::graphics_renderer::ShapeBatchRenderer,
    sprite_ren:     renderer::graphics_renderer::SpriteBatchRenderer,

    frame_buffer_listener: bus::BusReader::<(u32, u32)>,

    frame_width:      u32,
    frame_height:     u32,

    dw:             DrawingInformation,
    last_draw:      LastDraw,
}

impl Graphics3D {
    pub fn new(win: &mut window::Window) -> Graphics3D {
        Graphics3D {
            has_texture: false,
            texture: texture::TextureRegion::new_invalid(),
            font:   font::Font::new("res/fonts/arial.ttf", 64),

            shape_ren: ShapeBatchRenderer::new(Shader::from_file("res/shaders/graphics/shape3d.glsl"), {
                let mut vbl = VertexBufferLayout::new();
                vbl.push_f32(3);
                vbl.push_f32(3);
                vbl.push_f32(1);
                vbl.push_f32(4);
                vbl.push_f32(4);
                vbl.push_f32(4);
                vbl.push_f32(4);
                vbl
            }),
            sprite_ren: SpriteBatchRenderer::new(Shader::from_file("res/shaders/graphics/sprite3d.glsl"), {
                let mut vbl = VertexBufferLayout::new();
                vbl.push_f32(4);
                vbl.push_f32(4);
                vbl.push_f32(1); //TODO FIX CORRECT!
                vbl.push_f32(3);
                vbl.push_f32(3);
                vbl.push_f32(3);
                vbl
            }),
            
            frame_buffer_listener: win.create_frame_buffer_listener(),

            frame_width: win.get_width() as u32,
            frame_height: win.get_height() as u32,

            dw: DrawingInformation::new(),
            last_draw: LastDraw::None,
        }
    }

    pub fn update(&mut self) {
        let mut loop_done = false;
        while !loop_done {
            match self.frame_buffer_listener.try_recv() {
                Ok((width, height)) => {
                    self.frame_width = width;
                    self.frame_height = height;
                    info_log!("message: &str");
                },
                Err(_) => loop_done = true
            }
        }
    }

    pub fn flush(&mut self) {
        match self.last_draw {
            LastDraw::Plane => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 0);
                self.shape_ren.flush();
            },
            LastDraw::Line => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 1);
                self.shape_ren.flush();
            },
            LastDraw::Sphere => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 2);
                self.shape_ren.flush();
            },
            LastDraw::Cube => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 3);
                self.shape_ren.flush();
            },
            LastDraw::None => {}
            _ => {
                self.sprite_ren.flush(); //TODO FIX FOR ALL TEXTURED OBJECTS
            },
        }
        
        self.last_draw = LastDraw::None;
    }

    fn should_flush(&mut self, t: LastDraw) {
        if t == self.last_draw {
            return;
        }
        match self.last_draw {
            LastDraw::None => {
                self.last_draw = t;
            },
            _ => {
                self.flush();
                self.last_draw = t;
            }
        }
    }

    pub fn font(&self) -> &font::Font {
        &self.font
    }

    pub fn set_font(&mut self, font: font::Font) {
        self.font = font;
    }

    pub fn set_color(&mut self, color: Color) {
        self.dw.color = color;
    }

    pub fn clear(&mut self, color: Color) {
        self.flush();
        unsafe {
            renderer::std_renderer::set_clear_color(color);
            renderer::std_renderer::clear(renderer::std_renderer::ClearTarget::Color);
        }
    }

    pub fn line_width(&mut self, width: f32) {
        if self.last_draw == LastDraw::Line {
            self.flush();
        }
        self.dw.line_width = width;
        self.shape_ren.shader.bind();
        self.shape_ren.shader.upload_from_name_1f("u_line_width", width);
        Shader::un_bind();
    }

    pub fn sphere_detail(&mut self, detail_level: u32) {
        if detail_level > 127 {
            error_log!("Cannot set ellpise detail to: {} maximum is 127!", detail_level);
            return;
        }
        if self.last_draw == LastDraw::Sphere {
            self.flush();
        }
        self.dw.sphere_detail = detail_level;
        self.shape_ren.shader.bind();
        self.shape_ren.shader.upload_from_name_1i("u_sphere_detail", detail_level as i32);
        Shader::un_bind();
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, mat: &matrix::Mat4x4f) {
        let x1 = x1 * self.dw.scale.0 + self.dw.translation.0;
        let x2 = x2 * self.dw.scale.0 + self.dw.translation.0;
        let y1 = y1 * self.dw.scale.1 + self.dw.translation.1;
        let y2 = y2 * self.dw.scale.1 + self.dw.translation.1;
        let z1 = z1 * self.dw.scale.2 + self.dw.translation.2;
        let z2 = z2 * self.dw.scale.2 + self.dw.translation.2;
        
        let mut vertices = vec!(
            x1, y1, z1, x2, y2, z2,
            f32::from(self.dw.color),
        );

        unsafe { vertices.extend(mat.values.iter()); }

        self.should_flush(LastDraw::Line);
        self.shape_ren.add_vertex_data(&vertices);
    }

    pub fn draw_string(&mut self, text: &str, x: f32, y: f32, z: f32, mat: &matrix::Mat4x4f) {
        let sprite_texture = self.texture.clone();
        let mut current_advance = 0.0;

        for c in text.chars() {
            if c == ' ' {
                current_advance += self.font.width() as f32 / self.frame_width as f32 / 2.0;
                continue;
            }

            let glyph = &self.font.get_glyph(c);
            let texture = glyph.texture().clone();
            let bearing = glyph.bearing();
            let advance = glyph.advance();
            let size = glyph.size();

            //info_log!(("{}", bearing.1));

            let x = (x * self.frame_width as f32).round() / self.frame_width as f32 + bearing.0 / self.frame_width as f32 + current_advance;
            let y = (y * self.frame_height as f32).round() / self.frame_height as f32 + bearing.1 / self.frame_height as f32 - size.1 / self.frame_height as f32;
            let width = size.0 / self.frame_width as f32;
            let height = size.1 / self.frame_height as f32;

            current_advance += advance as f32 / self.frame_width as f32;

            self.texture(texture);
            self.fill_plane_with_texture(x, y, z, width, height, mat);
        }

        self.texture(sprite_texture);
    }
    
    fn fill_plane_with_texture(&mut self, x: f32, y: f32, z: f32, width: f32, height: f32, mat: &matrix::Mat4x4f) {
        let coords = self.texture.norm();
        let uvx = coords.0;
        let uvw = coords.2;
        let uvy = coords.1;
        let uvh = coords.3;
        let mut vertices = vec!(
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, z * self.dw.scale.2 + self.dw.translation.2,
            width * self.dw.scale.0, height * self.dw.scale.1,
            uvx, uvy, uvw, uvh,
            f32::from(self.dw.color),
        );

        unsafe { vertices.extend(mat.values.iter()); }


        self.should_flush(LastDraw::SpritePlane);
        self.sprite_ren.add_vertex_data(&vertices)
    }

    fn fill_plane_no_texture(&mut self, x: f32, y: f32, z: f32, width: f32, height: f32, mat: &matrix::Mat4x4f) {
        let vertices = [
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, z * self.dw.scale.2 + self.dw.translation.2,
            width * self.dw.scale.0, height * self.dw.scale.1, 0.0,
            f32::from(self.dw.color),
        ];
        
        //unsafe { vertices.extend_from_slice(&mat.values); }

        self.should_flush(LastDraw::Plane);
        self.shape_ren.add_vertex_data(&vertices);
        self.shape_ren.add_vertex_data(unsafe { &mat.values });
    }

    pub fn fill_plane(&mut self, x: f32, y: f32, z: f32, width: f32, height: f32, mat: &matrix::Mat4x4f) {
        if self.has_texture {
            self.fill_plane_with_texture(x, y, z, width, height, mat);
        }
        else {
            self.fill_plane_no_texture(x, y, z, width, height, mat);
        }
    }

    fn fill_cube_with_texture(&mut self, x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32, mat: &matrix::Mat4x4f) {
        let coords = self.texture.norm();
        let uvx = coords.0;
        let uvw = coords.2;
        let uvy = coords.1;
        let uvh = coords.3;
        let mut vertices = vec!(
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, z * self.dw.scale.2 + self.dw.translation.2,
            width * self.dw.scale.0, height * self.dw.scale.1, depth * self.dw.scale.2,
            uvx, uvy, uvw, uvh,
            f32::from(self.dw.color),
        );

        unsafe { vertices.extend(mat.values.iter()); }


        self.should_flush(LastDraw::SpriteCube);
        self.sprite_ren.add_vertex_data(&vertices)
    }

    fn fill_cube_no_texture(&mut self, x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32, mat: &matrix::Mat4x4f) {
        let vertices = [
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, z * self.dw.scale.2 + self.dw.translation.2,
            width * self.dw.scale.0, height * self.dw.scale.1, depth * self.dw.scale.2,
            f32::from(self.dw.color),
        ];
        
        //unsafe { vertices.extend_from_slice(&mat.values); }

        self.should_flush(LastDraw::Cube);
        self.shape_ren.add_vertex_data(&vertices);
        self.shape_ren.add_vertex_data(unsafe { &mat.values });
    }

    pub fn fill_cube(&mut self, x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32, mat: &matrix::Mat4x4f) {
        if self.has_texture {
            self.fill_cube_with_texture(x, y, z, width, height, depth, mat);
        }
        else {
            self.fill_cube_no_texture(x, y, z, width, height, depth, mat);
        }
    }

    pub fn fill_sphere(&mut self, x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32, mat: &matrix::Mat4x4f) {
        let mut vertices = vec!(
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, z * self.dw.scale.2 + self.dw.translation.2,
            width * self.dw.scale.0, height * self.dw.scale.1, depth * self.dw.scale.2,
            f32::from(self.dw.color),
        );

        unsafe { vertices.extend(mat.values.iter()); }

        self.should_flush(LastDraw::Sphere);
        self.shape_ren.add_vertex_data(&vertices)
    }

    pub fn texture(&mut self, texture: texture::TextureRegion) {
        if !self.texture.has_same_texture(&texture) && self.has_texture {
            self.flush();
        }
        if texture.is_valid() {
            self.sprite_ren.texture = texture.clone();
            self.texture = texture;
            self.has_texture = true;
        }
        else {
            self.sprite_ren.texture = texture.clone();
            self.texture = texture;
            self.has_texture = false;
        }
    }

    pub fn clear_texture(&mut self) {
        if self.has_texture {
            self.texture(texture::TextureRegion::new_invalid());
        }
    }

    pub fn set_sprite_shader(&mut self, shader: Shader)     { self.sprite_ren.shader = shader;  }
    pub fn set_shape_shader(&mut self, shader: Shader)      { self.shape_ren.shader = shader;   }

    pub fn frame_width(&self) -> u32  { self.frame_width    }
    pub fn frame_height(&self) -> u32 { self.frame_height   }
}