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

pub mod std_renderer {

    use crate::color::Color;

    #[repr(u32)]
    pub enum RenderingPrimitive {
        Triangles = gl::TRIANGLES,
        Lines = gl::LINES,
        Points = gl::POINTS,
    }

    #[repr(u32)]
    pub enum Capability {
        Blending = gl::BLEND,
		DepthTest = gl::DEPTH_TEST,
    }

    #[repr(u32)]
    pub enum BlendMode {
        SrcColor =			gl::SRC_COLOR,
		OneMinusSrcColor =	gl::ONE_MINUS_SRC_COLOR,
		DstColor =			gl::DST_COLOR,
		OneMinusDSTColor =	gl::ONE_MINUS_DST_COLOR,
		SrcAlpha =			gl::SRC_ALPHA,
		OneMinusSrcAlpha =	gl::ONE_MINUS_SRC_ALPHA,
		DstAlpha =			gl::DST_ALPHA,
        OneMinusDstAlpha =	gl::ONE_MINUS_DST_ALPHA,
        One =               gl::ONE,
    }

    #[repr(u32)]
    pub enum ClearTarget {
        Color = gl::COLOR_BUFFER_BIT,
        Depth = gl::DEPTH_BUFFER_BIT,
        Stencil = gl::STENCIL_BUFFER_BIT,
    }

    pub unsafe fn draw_elements(primitive: RenderingPrimitive, n_vertices: i32) { gl_call!(gl::DrawElements(primitive as u32, n_vertices, gl::UNSIGNED_INT, 0 as *const std::ffi::c_void)); }
    pub unsafe fn draw_array(primitve: RenderingPrimitive, n_vertices: i32)     { gl_call!(gl::DrawArrays(primitve as u32, 0, n_vertices)); }
    pub unsafe fn line_width(width: f32)                                        { gl_call!(gl::LineWidth(width)); }

    pub unsafe fn enable(cap: Capability) { gl_call!(gl::Enable(cap as u32)); }
	pub unsafe fn disable(cap: Capability) { gl_call!(gl::Disable(cap as u32)); }
    pub unsafe fn blend_func(sfactor: BlendMode, dfactor: BlendMode) { gl_call!(gl::BlendFunc(sfactor as u32, dfactor as u32)); }
    
    pub unsafe fn clear(target: ClearTarget) { gl_call!(gl::Clear(target as u32)); }
    
    pub unsafe fn set_clear_color(rgba: Color) { 
        let (r, g, b, a) = <(f32, f32, f32, f32)>::from(rgba);
        gl_call!(gl::ClearColor(r, g, b, a)); 
    }
}

pub mod graphics_renderer {
    use crate::shader;
    use crate::batch;
    use crate::texture;
    use crate::buffer;
    use crate::renderer;
    pub struct ShapeBatchRenderer {
        pub shader: shader::Shader,
        batch: batch::Batch,
        pub layout: buffer::VertexBufferLayout,
    }

    impl ShapeBatchRenderer {
        pub fn new(shader: shader::Shader, layout: buffer::VertexBufferLayout) -> ShapeBatchRenderer {
            ShapeBatchRenderer {
                shader,
                batch: batch::Batch::new(256),
                layout,
            }
        }

        pub fn flush(&mut self) {
            let batch = self.batch.get(&self.layout);        
            batch.0.bind();
    
            self.shader.bind();
    
            unsafe { renderer::std_renderer::draw_array(renderer::std_renderer::RenderingPrimitive::Points, batch.2 as i32); }
    
            buffer::VertexArray::un_bind();
            shader::Shader::un_bind();
        }

        pub fn add_vertex_data(&mut self, vertex_data: &[f32]) {
            if self.batch.capacity() - self.batch.len() < vertex_data.len() {
                let data = self.batch.as_mut_vec();
                data.reserve(data.capacity());
            }
            self.batch.add_vertex_data(vertex_data);
        }
    }

    pub struct SpriteBatchRenderer {
        pub shader: shader::Shader,
        pub texture: texture::TextureRegion,
        batch: batch::Batch,
        pub layout: buffer::VertexBufferLayout,
    }

    impl SpriteBatchRenderer {
        pub fn new(shader: shader::Shader, layout: buffer::VertexBufferLayout) -> SpriteBatchRenderer {
            SpriteBatchRenderer {
                shader,
                batch: batch::Batch::new(256),
                texture: texture::TextureRegion::new_invalid(),
                layout,
            }
        }

        pub fn flush(&mut self) {
            if !self.texture.is_valid() {
                return;
            }

            let batch = self.batch.get(&self.layout);        
            batch.0.bind();
    
            self.shader.bind();
            self.texture.bind(0);
    
            unsafe { renderer::std_renderer::draw_array(renderer::std_renderer::RenderingPrimitive::Points, batch.2 as i32); }
    
            buffer::VertexArray::un_bind();
            shader::Shader::un_bind();
            texture::Texture::un_bind();
        }

        pub fn add_vertex_data(&mut self, vertex_data: &[f32]) {
            if self.batch.capacity() - self.batch.len() < vertex_data.len() {
                let data = self.batch.as_mut_vec();
                data.reserve(data.capacity());
            }
            self.batch.add_vertex_data(vertex_data);
        }
    }
}

use engine_core::{error_log, fatal_log, window};

pub fn flush() {
    unsafe { gl_call!(gl::Flush()); }
}

pub fn init_gl(window: &mut window::Window) {
    if gl_loader::init_gl() == 0 {
        fatal_log!("Failed to load the opengl library!!!");
    }
    gl::load_with(|s| window.glfw_window.get_proc_address(s));
}

pub fn gl_clear_error() { unsafe {while gl::GetError() != gl::NO_ERROR {}}}

pub fn gl_log_call(file: &str, line: u32) -> bool {
    unsafe {
        let mut error_free = true;
        loop {
            let error = gl::GetError();
            if error == 0 { break; }

            let error_string;
            match error {
                0x0500 => error_string = "GL_INVALID_ENUM",
                0x0501 => error_string = "GL_INVALID_VALUE",
                0x0502 => error_string = "GL_INVALID_OPERATION",
                0x0503 => error_string = "GL_STACK_OVERFLOW",
                0x0504 => error_string = "GL_STACK_UNDERFLOW",
                0x0505 => error_string = "GL_OUT_OF_MEMORY",
                0x0506 => error_string = "GL_INVALID_FRAMEBUFFER_OPERATION",
                0x0507 => error_string = "GL_CONTEXT_LOST",
                0x8031 => error_string = "GL_TABLE_TOO_LARGE",
                _      => error_string = "Invalid GL error!"
            }
            
            error_log!("[OpenGL] {} {} : line {}", error_string, file, line);
            error_free = false;
        }
        return error_free;    
    }
}

pub fn gl_string(gl_enum: u32) -> String {
    unsafe {
        let mut ptr = gl::GetString(gl_enum);
        let mut length = 0;
        while *ptr != 0 {
            length += 1;
            ptr = ptr.offset(1);
        }
        ptr = ptr.offset(-length);

        let mut data = Vec::with_capacity(length as usize);
        for i in 0..length {
            data.push(*ptr.offset(i));
        }

        let message = String::from_utf8(data).unwrap();
        message
    }
}

pub fn gl_version() -> String {
    gl_string(gl::VERSION)
}