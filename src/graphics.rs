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
    Rect,
    Sprite,
    Line,
    Ellipse,
    Triangle,
}

struct DrawingInformation {
    color:         Color,
    translation:        (f32, f32),
    scale:              (f32, f32),
    line_width:         f32,
    ellipse_detail:     u32,
}

impl DrawingInformation {
    pub fn new() -> DrawingInformation {
        DrawingInformation {
            color: Color::from(0xFF_FF_FF_FF),
            translation: (0.0, 0.0),
            scale: (1.0, 1.0),
            line_width: 0.01,
            ellipse_detail: 100,
        }
    }
}

pub struct Graphics {
    has_texture:    bool,
    texture:        texture::TextureRegion, 
    font:           font::Font,
    
    shape_ren:      ShapeBatchRenderer,
    sprite_ren:     SpriteBatchRenderer,

    frame_buffer_listener: bus::BusReader::<(u32, u32)>,

    frame_width:      u32,
    frame_height:     u32,

    dw:             DrawingInformation,
    last_draw:      LastDraw,
}

impl Graphics {
    pub fn new(win: &mut window::Window) -> Graphics {
        Graphics {
            has_texture: false,
            texture: texture::TextureRegion::new_invalid(),

            font:   font::Font::new_invalid(),

            //shape_ren: renderer::graphics_renderer::ShapeRenderer::new(shader::Shader::from_source(SHAPE_SHADER_SOURCE)),
            //sprite_ren: renderer::graphics_renderer::SpriteRenderer::new(shader::Shader::from_source(SPRITE_SHADER_SOURCE)),

            shape_ren: ShapeBatchRenderer::new(Shader::from_source(SHAPE_SHADER_SOURCE), {
                let mut vbl = VertexBufferLayout::new();
                vbl.push_f32(4);
                vbl.push_f32(1);
                vbl
            }),
            sprite_ren: SpriteBatchRenderer::new(Shader::from_source(SPRITE_SHADER_SOURCE), {
                let mut vbl = VertexBufferLayout::new();
                vbl.push_f32(4);
                vbl.push_f32(4);
                vbl.push_f32(1);
                vbl
            }),
            
            frame_buffer_listener: win.create_frame_buffer_listener(),

            frame_width: win.get_width() as u32,
            frame_height: win.get_height() as u32,

            dw: DrawingInformation::new(),
            last_draw: LastDraw::None,
        }
    }

    pub fn from(win: &mut window::Window, shape_ren: Shader, sprite_ren: Shader, font: font::Font) -> Graphics {
        Graphics {
            has_texture: false,
            texture: texture::TextureRegion::new_invalid(),
            font: font,

            shape_ren: ShapeBatchRenderer::new(shape_ren, {
                let mut vbl = VertexBufferLayout::new();
                vbl.push_f32(4);
                vbl.push_f32(1);
                vbl
            }),
            sprite_ren: SpriteBatchRenderer::new(sprite_ren, {
                let mut vbl = VertexBufferLayout::new();
                vbl.push_f32(4);
                vbl.push_f32(4);
                vbl.push_f32(1);
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
                    unsafe { gl_call!(gl::Viewport(0, 0, width as i32, height as i32)); };
                    self.frame_width = width;
                    self.frame_height = height;
                },
                Err(_) => loop_done = true
            }
        }
    }

    pub fn flush(&mut self) {
        match self.last_draw {
            LastDraw::Rect => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 0);
                self.shape_ren.flush();
            },
            LastDraw::Triangle => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 1);
                self.shape_ren.flush();
            },
            LastDraw::Ellipse => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 2);
                self.shape_ren.flush();
            },
            LastDraw::Line => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 3);
                self.shape_ren.flush();
            },
            LastDraw::Sprite => {
                self.sprite_ren.flush();
            },
            LastDraw::None => {}
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

    pub fn ellipse_detail(&mut self, detail_level: u32) {
        if detail_level > 127 {
            error_log!("Cannot set ellpise detail to: {} maximum is 127!", detail_level);
            return;
        }
        if self.last_draw == LastDraw::Ellipse {
            self.flush();
        }
        self.dw.ellipse_detail = detail_level;
        self.shape_ren.shader.bind();
        self.shape_ren.shader.upload_from_name_1i("u_ellipse_detail", detail_level as i32);
        Shader::un_bind();
    }

    pub fn set_translation(&mut self, x: f32, y: f32) {
        self.dw.translation = (x, y);
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.dw.scale = (x, y);
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let x1 = x1 * self.dw.scale.0 + self.dw.translation.0;
        let x2 = x2 * self.dw.scale.0 + self.dw.translation.0;
        let y1 = y1 * self.dw.scale.1 + self.dw.translation.1;
        let y2 = y2 * self.dw.scale.1 + self.dw.translation.1;
        let vertices = vec!(
            x1, y1, x2, y2,
            f32::from(self.dw.color),
        );

        self.should_flush(LastDraw::Line);
        self.shape_ren.add_vertex_data(&vertices);
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.draw_line(x - self.dw.line_width, y, x + width + self.dw.line_width, y);
        self.draw_line(x, y - self.dw.line_width, x, y + height + self.dw.line_width);
        self.draw_line(x + width, y - self.dw.line_width, x + width, y + height + self.dw.line_width);
        self.draw_line(x - self.dw.line_width, y + height, x + width + self.dw.line_width, y + height);
    }

    /*fn fill_quad_no_texture(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) {
        let mut vertices = vec!(
            x1, y1, self.dw.color_rgba, //(0, 0)
            x2, y2, self.dw.color_rgba, //(1, 0)
            x3, y3, self.dw.color_rgba, //(0, 1)
    
            x2, y2, self.dw.color_rgba, //(1, 0)
            x4, y4, self.dw.color_rgba, //(1, 1)
            x3, y3, self.dw.color_rgba, //(0, 1)
        );

        let mut i = 0;
        while i < vertices.len()
        {
            vertices[i] = vertices[i] * self.dw.scale.0 + self.dw.translation.0;
            vertices[i + 1] = vertices[i + 1] * self.dw.scale.1 + self.dw.translation.1;

            i += 3;
        }

        self.should_flush(LastDraw::Shape);
        self.shape_ren.add_vertex_data(vertices);
    }*/

    /*fn fill_quad_with_texture(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) {
        let coords = self.texture.tex_coords();
        let uvx1 = coords.0;
        let uvx2 = coords.2;
        let uvy1 = coords.1;
        let uvy2 = coords.3;

        let mut vertices = vec!(
            x1, y1, self.dw.color_rgba, uvx1, uvy1, //(0, 0) 
            x2, y2, self.dw.color_rgba, uvx2, uvy1, //(1, 0)
            x3, y3, self.dw.color_rgba, uvx1, uvy2, //(0, 1)
    
            x2, y2, self.dw.color_rgba, uvx2, uvy1, //(1, 0)
            x4, y4, self.dw.color_rgba, uvx2, uvy2, //(1, 1)
            x3, y3, self.dw.color_rgba, uvx1, uvy2, //(0, 1)
        );

        let mut i = 0;
        while i < vertices.len()
        {
            vertices[i] = vertices[i] * self.dw.scale.0 + self.dw.translation.0;
            vertices[i + 1] = vertices[i + 1] * self.dw.scale.1 + self.dw.translation.1;

            i += 5;
        }

        self.should_flush(LastDraw::Texture);
        self.sp.add_vertex_data(vertices);
    }*/

    #[deprecated]
    pub fn fill_quad(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _x3: f32, _y3: f32, _x4: f32, _y4: f32) {
        if self.has_texture {
            //self.fill_quad_with_texture(x1, y1, x2, y2, x3, y3, x4, y4);
            warn_log!("Tried to draw unsupported textured quad!");
        }
        else {
            //self.fill_quad_no_texture(x1, y1, x2, y2, x3, y3, x4, y4);
            warn_log!("Tried to draw unsupported colored quad!");
        }
    }

    fn fill_rect_with_texture(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let coords = self.texture.norm();
        let uvx = coords.0;
        let uvw = coords.2;
        let uvy = coords.1;
        let uvh = coords.3;
        let vertices = [
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, width * self.dw.scale.0, height * self.dw.scale.1,
            uvx, uvy, uvw, uvh,
            f32::from(self.dw.color),
        ];

        self.should_flush(LastDraw::Sprite);
        self.sprite_ren.add_vertex_data(&vertices)
    }

    fn fill_rect_no_texture(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let vertices = [
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, width * self.dw.scale.0, height * self.dw.scale.1,
            f32::from(self.dw.color),
        ];

        self.should_flush(LastDraw::Rect);
        self.shape_ren.add_vertex_data(&vertices)
    }

    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if self.has_texture {
            self.fill_rect_with_texture(x, y, width, height);
        }
        else {
            self.fill_rect_no_texture(x, y, width, height);
        }
    }

    pub fn fill_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let vertices = vec!(
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, width * self.dw.scale.0, height * self.dw.scale.1,
            f32::from(self.dw.color),
        );

        self.should_flush(LastDraw::Ellipse);
        self.shape_ren.add_vertex_data(&vertices)
    }

    pub fn draw_string(&mut self, text: &str, x: f32, y: f32) {
        let sprite_texture = self.texture.clone();
        let mut current_advance = 0.0;

        for c in text.chars() {
            if c == ' ' {
                current_advance += self.font.width() as f32 / self.frame_width as f32 / 2.0;
                continue;
            }

            let glyph = match self.font.glyph(c) {
                Some(g) => g,
                None => match self.font.glyph('?') {
                    Some(g) => g,
                    None => {
                        current_advance += self.font.width() as f32 / self.frame_width as f32 / 2.0;
                        continue;
                    }
                },
            };

            let texture = glyph.texture().clone();
            let bearing = glyph.bearing();
            let advance = glyph.advance();
            let size = glyph.size();

            let x = ((x + bearing.0 / self.frame_width as f32 + current_advance) * self.frame_width as f32).round() / self.frame_width as f32;
            let y = ((y + bearing.1 / self.frame_height as f32 - size.1 / self.frame_height as f32) * self.frame_height as f32).round() / self.frame_height as f32;
            let width = size.0 / self.frame_width as f32;
            let height = size.1 / self.frame_height as f32;

            current_advance += advance as f32 / self.frame_width as f32;

            self.texture(texture);
            self.fill_rect_with_texture(x, y, width, height);
        }

        self.texture(sprite_texture);
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
    pub fn sprite_shader(&mut self) -> &mut Shader { &mut self.sprite_ren.shader }
    pub fn shape_shader (&mut self) -> &mut Shader { &mut self.shape_ren.shader  }
    
    pub fn frame_width(&self) -> u32  { self.frame_width    }
    pub fn frame_height(&self) -> u32 { self.frame_height   }
}

pub struct Graphics2D {
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

impl Graphics2D {
    pub fn new(win: &mut window::Window) -> Graphics2D {
        Graphics2D {
            has_texture: false,
            texture: texture::TextureRegion::new_invalid(),
            font:   font::Font::new_invalid(),

            shape_ren: ShapeBatchRenderer::new(Shader::from_file("res/shaders/graphics/shape2d.glsl"), {
                let mut vbl = VertexBufferLayout::new();
                vbl.push_f32(4);
                vbl.push_f32(1);
                vbl.push_f32(3);
                vbl.push_f32(3);
                vbl.push_f32(3);
                vbl
            }),
            sprite_ren: SpriteBatchRenderer::new(Shader::from_file("res/shaders/graphics/sprite2d.glsl"), {
                let mut vbl = VertexBufferLayout::new();
                vbl.push_f32(4);
                vbl.push_f32(4);
                vbl.push_f32(1);
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
                    unsafe { gl_call!(gl::Viewport(0, 0, width as i32, height as i32)); };
                    self.frame_width = width;
                    self.frame_height = height;
                    //info_log!("message: &str");
                },
                Err(_) => loop_done = true
            }
        }
    }

    pub fn flush(&mut self) {
        match self.last_draw {
            LastDraw::Rect => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 0);
                self.shape_ren.flush();
            },
            LastDraw::Triangle => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 1);
                self.shape_ren.flush();
            },
            LastDraw::Ellipse => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 2);
                self.shape_ren.flush();
            },
            LastDraw::Line => {
                self.shape_ren.shader.bind();
                self.shape_ren.shader.upload_from_name_1i("u_primitive", 3);
                self.shape_ren.flush();
            },
            LastDraw::Sprite => {
                self.sprite_ren.flush();
            },
            LastDraw::None => {}
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

    pub fn ellipse_detail(&mut self, detail_level: u32) {
        if detail_level > 127 {
            error_log!("Cannot set ellpise detail to: {} maximum is 127!", detail_level);
            return;
        }
        if self.last_draw == LastDraw::Ellipse {
            self.flush();
        }
        self.dw.ellipse_detail = detail_level;
        self.shape_ren.shader.bind();
        self.shape_ren.shader.upload_from_name_1i("u_ellipse_detail", detail_level as i32);
        Shader::un_bind();
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, mat: &matrix::Mat3x3f) {
        let x1 = x1 * self.dw.scale.0 + self.dw.translation.0;
        let x2 = x2 * self.dw.scale.0 + self.dw.translation.0;
        let y1 = y1 * self.dw.scale.1 + self.dw.translation.1;
        let y2 = y2 * self.dw.scale.1 + self.dw.translation.1;
        
        let mut vertices = vec!(
            x1, y1, x2, y2,
            f32::from(self.dw.color),
        );

        unsafe { vertices.extend(mat.values.iter()); }

        self.should_flush(LastDraw::Line);
        self.shape_ren.add_vertex_data(&vertices);
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, mat: &matrix::Mat3x3f) {
        self.draw_line(x - self.dw.line_width, y, x + width + self.dw.line_width, y, mat);
        self.draw_line(x, y - self.dw.line_width, x, y + height + self.dw.line_width, mat);
        self.draw_line(x + width, y - self.dw.line_width, x + width, y + height + self.dw.line_width, mat);
        self.draw_line(x - self.dw.line_width, y + height, x + width + self.dw.line_width, y + height, mat);
    }

    pub fn draw_string(&mut self, text: &str, x: f32, y: f32, mat: &matrix::Mat3x3f) {
        let sprite_texture = self.texture.clone();
        let mut current_advance = 0.0;

        for c in text.chars() {
            if c == ' ' {
                current_advance += self.font.width() as f32 / self.frame_width as f32 / 2.0;
                continue;
            }

            let glyph = match self.font.glyph(c) {
                Some(g) => g,
                None => match self.font.glyph('?') {
                    Some(g) => g,
                    None => {
                        current_advance += self.font.width() as f32 / self.frame_width as f32 / 2.0;
                        continue;
                    }
                },
            };

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
            self.fill_rect_with_texture(x, y, width, height, mat);
        }

        self.texture(sprite_texture);
    }  

    fn fill_rect_with_texture(&mut self, x: f32, y: f32, width: f32, height: f32, mat: &matrix::Mat3x3f) {
        let coords = self.texture.norm();
        let uvx = coords.0;
        let uvw = coords.2;
        let uvy = coords.1;
        let uvh = coords.3;
        let mut vertices = vec!(
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, width * self.dw.scale.0, height * self.dw.scale.1,
            uvx, uvy, uvw, uvh,
            f32::from(self.dw.color),
        );

        unsafe { vertices.extend(mat.values.iter()); }


        self.should_flush(LastDraw::Sprite);
        self.sprite_ren.add_vertex_data(&vertices)
    }

    fn fill_rect_no_texture(&mut self, x: f32, y: f32, width: f32, height: f32, mat: &matrix::Mat3x3f) {
        let vertices = [
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, width * self.dw.scale.0, height * self.dw.scale.1,
            f32::from(self.dw.color),
        ];
        
        //unsafe { vertices.extend_from_slice(&mat.values); }

        self.should_flush(LastDraw::Rect);
        self.shape_ren.add_vertex_data(&vertices);
        self.shape_ren.add_vertex_data(unsafe { &mat.values });
    }

    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, mat: &matrix::Mat3x3f) {
        if self.has_texture {
            self.fill_rect_with_texture(x, y, width, height, mat);
        }
        else {
            self.fill_rect_no_texture(x, y, width, height, mat);
        }
    }

    pub fn fill_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32, mat: &matrix::Mat3x3f) {
        let mut vertices = vec!(
            x * self.dw.scale.0 + self.dw.translation.0, y * self.dw.scale.1 + self.dw.translation.1, width * self.dw.scale.0, height * self.dw.scale.1,
            f32::from(self.dw.color),
        );

        unsafe { vertices.extend(mat.values.iter()); }

        self.should_flush(LastDraw::Ellipse);
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
    pub fn sprite_shader(&mut self) -> &mut Shader { &mut self.sprite_ren.shader }
    pub fn shape_shader (&mut self) -> &mut Shader { &mut self.shape_ren.shader  }

    pub fn frame_width(&self) -> u32  { self.frame_width    }
    pub fn frame_height(&self) -> u32 { self.frame_height   }
}

const SHAPE_SHADER_SOURCE: &str = "
#shader vertex
#version 330 core

layout(location = 0) in vec4 v_bounds;
layout(location = 1) in int v_color;

out int tight_color;

void main() {
    gl_Position = v_bounds;
    tight_color = v_color;
}

#shader geometry
#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 256) out;

in int tight_color[];

out vec4 color;

const int DRAW_RECT = 0;
const int DRAW_TRIANGLE = 1;
const int DRAW_ELLIPSE = 2;
const int DRAW_LINE = 3;

uniform int u_primitive = 0;
uniform int u_ellipse_detail = 100;
uniform float u_line_width = 0.01;


vec4 normal_color(int tight_color) {
    float a = tight_color & 255;
    float b = (tight_color >> 8) & 255;
    float g = (tight_color >> 16) & 255;
    float r = (tight_color >> 24) & 255;
    return vec4(r / 255, g / 255, b / 255, a / 255);
}

void draw_rect(float x, float y, float width, float height) {
    //(0, 0)
    gl_Position = vec4(x, y, 1.0, 1.0);
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(x + width, y, 1.0, 1.0);
    EmitVertex();

    //(0, 1)
    gl_Position = vec4(x, y + height, 1.0, 1.0);
    EmitVertex();

    //(1, 1)
    gl_Position = vec4(x + width, y + height, 1.0, 1.0);
    EmitVertex();
}

void draw_triangle(float x, float y, float width, float height) {
    //(0.5, 0)
    gl_Position = vec4(x + width / 2.0, y + height, 1.0, 1.0);
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(x + width, y, 1.0, 1.0);
    EmitVertex();

    //(0, 0)
    gl_Position = vec4(x, y, 1.0, 1.0);
    EmitVertex();
}

void draw_ellipse(float x, float y, float width, float height) {
    for(int i = 0; i < u_ellipse_detail+1; i++) {
        float nx = cos(float(i) / float(u_ellipse_detail) * 2.0 * 3.14) * width / 2.0;
        float ny = sin(float(i) / float(u_ellipse_detail) * 2.0 * 3.14) * height / 2.0;
        nx = x + width / 2.0 + nx;
        ny = y + height / 2.0 + ny;

        gl_Position = vec4(nx, ny, 1.0, 1.0);
        EmitVertex();

        gl_Position = vec4(x + width / 2.0, y + height / 2.0, 1.0, 1.0);
        EmitVertex();
    }
    float nx = cos(float(0) / float(u_ellipse_detail) * 2.0 * 3.14) * width / 2.0;
    float ny = sin(float(0) / float(u_ellipse_detail) * 2.0 * 3.14) * height / 2.0;
    nx = x + width / 2.0 + nx;
    ny = y + height / 2.0 + ny;

    gl_Position = vec4(nx, ny, 1.0, 1.0);
    EmitVertex();

    gl_Position = vec4(x + width / 2.0, y + height / 2.0, 1.0, 1.0);
    EmitVertex();
}

void draw_line(float x1, float y1, float x2, float y2) {
    float x_len = x1 - x2;
    float y_len = y1 - y2;
    
    float a = atan(y_len / x_len);
    float pi = 3.14 / 2.0;

    vec2 pos1 = vec2(x1 + cos(a + pi) * u_line_width, y1 + sin(a + pi) * u_line_width); //(0, 0)
    vec2 pos2 = vec2(x1 + cos(a - pi) * u_line_width, y1 + sin(a - pi) * u_line_width); //(1, 0)
    vec2 pos3 = vec2(x2 + cos(a + pi) * u_line_width, y2 + sin(a + pi) * u_line_width); //(0, 1)
    vec2 pos4 = vec2(x2 + cos(a - pi) * u_line_width, y2 + sin(a - pi) * u_line_width); //(1, 1)

    ///////////////////////////////////////////////////////

    //(0, 0)
    gl_Position = vec4(pos1.x, pos1.y, 1.0, 1.0);
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(pos2.x, pos2.y, 1.0, 1.0);
    EmitVertex();

    //(0, 1)
    gl_Position = vec4(pos3.x, pos3.y, 1.0, 1.0);
    EmitVertex();

    //(1, 1)
    gl_Position = vec4(pos4.x, pos4.y, 1.0, 1.0);
    EmitVertex();
}

void main() {
    vec4 col = normal_color(tight_color[0]);
    vec4 pos = gl_in[0].gl_Position;

    color = col;

    ///////////////////////////////////////////////////////
    if(u_primitive == DRAW_ELLIPSE) {
        draw_ellipse(pos.x, pos.y, pos.z, pos.w);
    }
    else if(u_primitive == DRAW_LINE) {
        draw_line(pos.x, pos.y, pos.z, pos.w);
    }
    else if(u_primitive == DRAW_RECT) {
        draw_rect(pos.x, pos.y, pos.z, pos.w);
    }
    else if(u_primitive == DRAW_TRIANGLE) {
        draw_triangle(pos.x, pos.y, pos.z, pos.w);
    }
   
}

#shader fragment
#version 330 core

in vec4 color;

layout(location = 0) out vec4 out_color;

void main() {
	out_color = color;
}
";

const SPRITE_SHADER_SOURCE: &str = "
#shader vertex
#version 330 core

layout(location = 0) in vec4 v_bounds;
layout(location = 1) in vec4 v_uv_bounds;
layout(location = 2) in int v_color;

out vec4 uv_bounds;
out int tight_color;

void main() {
    gl_Position = v_bounds;
    uv_bounds = v_uv_bounds;
    tight_color = v_color;
}

#shader geometry
#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 6) out;

in vec4 uv_bounds[];
in int tight_color[];

out vec2 uv;
out vec4 color;

vec4 normal_color(int tight_color) {
    float a = tight_color & 255;
    float b = (tight_color >> 8) & 255;
    float g = (tight_color >> 16) & 255;
    float r = (tight_color >> 24) & 255;
    return vec4(r / 255, g / 255, b / 255, a / 255);
}

void main() {
    vec4 col = normal_color(tight_color[0]);
    vec4 pos = gl_in[0].gl_Position;

    color = col;

    ///////////////////////////////////////////////////////

    //pos.x *= 0.1;
    //pos.y *= 0.1;
    //pos.z
    

    //(0, 0)
    gl_Position = vec4(pos.x, pos.y, 1.0, 1.0);
    uv = vec2(uv_bounds[0].x, uv_bounds[0].y);
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(pos.x + pos.z, pos.y, 1.0, 1.0);
    uv = vec2(uv_bounds[0].x + uv_bounds[0].z, uv_bounds[0].y);
    EmitVertex();

    //(0, 1)
    gl_Position = vec4(pos.x, pos.y + pos.w, 1.0, 1.0);
    uv = vec2(uv_bounds[0].x, uv_bounds[0].y + uv_bounds[0].w);
    EmitVertex();

    //(1, 1)
    gl_Position = vec4(pos.x + pos.z, pos.y + pos.w, 1.0, 1.0);
    uv = vec2(uv_bounds[0].x + uv_bounds[0].z, uv_bounds[0].y + uv_bounds[0].w);
    EmitVertex();
}

#shader fragment
#version 330 core

in vec2 uv;
in vec4 color;

layout(location = 0) out vec4 out_color;

uniform sampler2D u_Texture;

void main() {
    vec4 texColor = texture(u_Texture, uv);
	out_color = texColor * color;
}
";