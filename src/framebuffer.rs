/*
 *   Copyright (c) 2021 Ludwig Bogsveen
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

use engine_core::{error_log, info_log};

use crate::texture::Texture;
use std::rc::Rc;

pub enum FrameBufferError {
    Undefined,
    IncompleteAttachment,
    IncompleteMissingAttachment,
    IncompleteDrawBuffer,
    IncompleteReadBuffer,
    Unsupported,
    IncompleteMultisample,
    IncompleteLayerTargets,
}

impl From<u32> for FrameBufferError {
    fn from(gl_error_code: u32) -> Self {
        match gl_error_code {
            gl::FRAMEBUFFER_UNDEFINED                       => Self::Undefined,
            gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT           => Self::IncompleteAttachment,
            gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT   => Self::IncompleteMissingAttachment,
            gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER          => Self::IncompleteDrawBuffer,
            gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER          => Self::IncompleteReadBuffer,
            gl::FRAMEBUFFER_UNSUPPORTED                     => Self::Unsupported,
            gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE          => Self::IncompleteMultisample,
            gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS        => Self::IncompleteLayerTargets,
            _ => {
                error_log!("Invalid Framebuffer error detected!");
                return Self::Undefined;
            }
        }
    }
}

pub struct FrameBuffer {
    gl_buffer_id: u32,
    texture: Rc<Texture>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> FrameBuffer {
        let texture = Texture::from_color(width, height, 0xFF_00_00_FF);
        texture.bind(0);

        unsafe {
            let mut gl_buffer_id = 0;
            gl_call!(gl::GenFramebuffers(1, &mut gl_buffer_id));
            gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, gl_buffer_id));

            
            gl_call!(gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, gl_buffer_id, 0));

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                let error = match FrameBufferError::from(gl::CheckFramebufferStatus(gl::FRAMEBUFFER)) {
                    FrameBufferError::Undefined                     => "UNDEFINED",
                    FrameBufferError::IncompleteAttachment          => "INCOMPLETE_ATTACHMENT",
                    FrameBufferError::IncompleteMissingAttachment   => "INCOMPLETE_MISSING_ATTACHMENT",
                    FrameBufferError::IncompleteDrawBuffer          => "INCOMPLETE_DRAW_BUFFER",
                    FrameBufferError::IncompleteReadBuffer          => "INCOMPLETE_READ_BUFFER",
                    FrameBufferError::Unsupported                   => "UNSUPPORTED",
                    FrameBufferError::IncompleteMultisample         => "INCOMPLETE_MULTISAMPLE",
                    FrameBufferError::IncompleteLayerTargets        => "INCOMPLETE_LAYER_TARGETS",
                };
                error_log!("Failed to create OpenGL Framebuffer object! ERROR CODE : {}", error);
            }
            
            
            Texture::un_bind();
            Self::un_bind();

            FrameBuffer {
                gl_buffer_id,
                texture,
            }
        }
    }

    pub fn get_pixels(&self, x: u32, y: u32, width: u32, height: u32) -> Vec<u8> {
		let mut data = vec![0; width as usize * height as usize * 4];
        self.bind();
		unsafe {
			gl_call!(gl::ReadPixels(x as i32, y as i32, width as i32, height as i32, gl::RGBA, gl::UNSIGNED_BYTE, data.as_mut_ptr() as *mut std::ffi::c_void));
		}
		data
	}

    pub fn texture(&self) -> &Rc<Texture> {
        &self.texture
    }

    pub fn get_pixels_standard_frame_buffer(x: u32, y: u32, width: u32, height: u32) -> Vec<u8> {
        let mut data = vec![0; width as usize * height as usize * 4];
        FrameBuffer::un_bind();
		unsafe {
			gl_call!(gl::ReadPixels(x as i32, y as i32, width as i32, height as i32, gl::RGBA, gl::UNSIGNED_BYTE, data.as_mut_ptr() as *mut std::ffi::c_void));
		}
		data
    }

    pub fn bind(&self) {
        unsafe {
            info_log!("A{}", self.gl_buffer_id);
            gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, self.gl_buffer_id));
        }
    }

    pub fn un_bind() {
        unsafe {
            info_log!("B0");
            gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, 0));
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl_call!(gl::DeleteFramebuffers(1, &self.gl_buffer_id));
        }
    }
}

