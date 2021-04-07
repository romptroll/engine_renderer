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

use std::convert::TryInto;

pub struct VertexBuffer { gl_buffer_id : u32 }
pub struct IndexBuffer  { gl_buffer_id : u32 }
pub struct VertexBufferElement {
    gl_type: u32,
    count: u32,
    normalized: u8
}
pub struct VertexBufferLayout {
    elements: std::vec::Vec<VertexBufferElement>,
	divisors: std::vec::Vec<(u32, u32)>,
	stride: u32
}
pub struct VertexArray {
    gl_buffer_id : u32,
    gl_attribute_index : u32
}
impl IndexBuffer {
    
    pub fn new(indices: &[u32]) -> IndexBuffer {
        unsafe {
            let mut buffer = IndexBuffer { gl_buffer_id: 0 };
            gl_call!(gl::GenBuffers(1, &mut buffer.gl_buffer_id));
            gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.gl_buffer_id));
            gl_call!(gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * 4).try_into().unwrap(), indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW));
            gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
            buffer
        }
    }

    pub unsafe fn from_ptr(indices: *const u32, size: isize) -> IndexBuffer {
        let mut buffer = IndexBuffer { gl_buffer_id: 0 };
        gl_call!(gl::GenBuffers(1, &mut buffer.gl_buffer_id));
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.gl_buffer_id));
        gl_call!(gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size * 4, indices as *const std::ffi::c_void, gl::STATIC_DRAW));
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
        buffer
    }
    
    pub unsafe fn get_sub_data(&self, size : isize, offset : isize) -> std::vec::Vec<u32> {
        let mut indices: std::vec::Vec<u32> = std::vec::Vec::with_capacity(size.try_into().unwrap());
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gl_buffer_id));
        gl_call!(gl::GetBufferSubData(gl::ELEMENT_ARRAY_BUFFER, offset * 4, size * 4, indices.as_mut_ptr() as *mut std::ffi::c_void));
        indices.set_len(size.try_into().unwrap());
        indices
    }
    
    pub unsafe fn sub_data(&self, indices: *const u32, size: isize, offset : isize) {
        gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gl_buffer_id));
        gl_call!(gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, offset * 4, size * 4, indices as *const std::ffi::c_void));
    }
    
    pub fn bind(&self)  { unsafe { gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gl_buffer_id)); }}
    pub fn un_bind()    { unsafe { gl_call!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0)); }}
}

impl Drop for IndexBuffer {fn drop(&mut self)  { unsafe {gl_call!(gl::DeleteBuffers(1, &self.gl_buffer_id)); }}}

impl VertexBuffer {
    pub unsafe fn from_ptr(vertices: *const f32, size: isize) -> VertexBuffer {
        let mut buffer = VertexBuffer { gl_buffer_id: 0 };
        gl_call!(gl::GenBuffers(1, &mut buffer.gl_buffer_id));
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, buffer.gl_buffer_id));
        gl_call!(gl::BufferData(gl::ARRAY_BUFFER, size * 4, vertices as *const std::ffi::c_void, gl::STATIC_DRAW));
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, 0));
        buffer
    }

    pub fn new(vertices: &[f32]) -> VertexBuffer {
        unsafe {
            let mut buffer = VertexBuffer { gl_buffer_id: 0 };
            gl_call!(gl::GenBuffers(1, &mut buffer.gl_buffer_id));
            gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, buffer.gl_buffer_id));
            gl_call!(gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * 4).try_into().unwrap(), vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW));
            gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, 0));
            buffer
        }
    }
    
    pub unsafe fn get_sub_data(&self, size : isize, offset : isize) -> std::vec::Vec<f32> {
        let mut vertices: std::vec::Vec<f32> = std::vec::Vec::with_capacity(size.try_into().unwrap());
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_buffer_id));
        gl_call!(gl::GetBufferSubData(gl::ARRAY_BUFFER, offset * 4, size * 4, vertices.as_mut_ptr() as *mut std::ffi::c_void));
        vertices.set_len(size.try_into().unwrap());
        vertices
    }
    
    pub unsafe fn sub_data(&self, vertices: *const f32, size: isize, offset : isize) {
        gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_buffer_id));
        gl_call!(gl::BufferSubData(gl::ARRAY_BUFFER, offset * 4, size * 4, vertices as *const std::ffi::c_void));
    }
    
    pub fn bind(&self)  { unsafe { gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_buffer_id)); }}
    pub fn un_bind()    { unsafe { gl_call!(gl::BindBuffer(gl::ARRAY_BUFFER, 0)); }}
}

impl Drop for VertexBuffer { fn drop(&mut self)  { unsafe {gl_call!(gl::DeleteBuffers(1, &self.gl_buffer_id)); }}}

impl VertexBufferElement {
    fn get_size_of_type(gl_type: u32) -> u32 {
        match gl_type {
            gl::FLOAT           => 4,
            gl::INT			    => 4,
            gl::BYTE			=> 1,
            gl::UNSIGNED_INT	=> 4,
            gl::UNSIGNED_BYTE	=> 1,
            _ => { assert!(false); 0},
        }
    }
}

impl VertexBufferLayout {
    pub fn new() -> VertexBufferLayout {
        let vbl = VertexBufferLayout {
            elements: std::vec::Vec::new(),
            divisors: std::vec::Vec::new(),
            stride: 0
        };
        vbl
    }

    pub fn push_f32(&mut self, count: u32) { 
        self.elements.push(VertexBufferElement { gl_type: gl::FLOAT, count: count, normalized: gl::FALSE });
        self.stride += count * VertexBufferElement::get_size_of_type(gl::FLOAT);
    }

    pub fn push_i32(&mut self, count: u32) { 
        self.elements.push(VertexBufferElement { gl_type: gl::INT, count: count, normalized: gl::FALSE });
        self.stride += count * VertexBufferElement::get_size_of_type(gl::INT);
    }

    pub fn push_i8(&mut self, count: u32) { 
        self.elements.push(VertexBufferElement { gl_type: gl::BYTE, count: count, normalized: gl::FALSE });
        self.stride += count * VertexBufferElement::get_size_of_type(gl::BYTE);
    }

    pub fn push_u32(&mut self, count: u32) { 
        self.elements.push(VertexBufferElement { gl_type: gl::UNSIGNED_INT, count: count, normalized: gl::FALSE });
        self.stride += count * VertexBufferElement::get_size_of_type(gl::UNSIGNED_INT);
    }

    pub fn push_u8(&mut self, count: u32) { 
        self.elements.push(VertexBufferElement { gl_type: gl::UNSIGNED_BYTE, count: count, normalized: gl::FALSE });
        self.stride += count * VertexBufferElement::get_size_of_type(gl::UNSIGNED_BYTE);
    }

    pub fn push_divisor(&mut self, index: u32, divisor: u32)    { self.divisors.push((index, divisor)) }

    pub fn get_elements(&self)  -> &std::vec::Vec<VertexBufferElement> { &self.elements }
    pub fn get_divisors(&self)  -> &std::vec::Vec<(u32, u32)> { &self.divisors }
    pub fn get_stride(&self)    -> u32 { self.stride }
}

impl VertexArray {
    pub fn new() -> VertexArray {
        unsafe {
            let mut array = VertexArray {gl_buffer_id: 0, gl_attribute_index: 0};
            gl_call!(gl::GenVertexArrays(1, &mut array.gl_buffer_id));
            array
        }
    }

    pub fn add_buffer(&mut self, vb : &VertexBuffer, vbl : &VertexBufferLayout) {
        unsafe {
            vb.bind();
            self.bind();
            let mut offset = 0;
            let stride: i32 = vbl.get_stride().try_into().unwrap();
            let elements = vbl.get_elements();
            for element in elements {
                gl_call!(gl::VertexAttribPointer(self.gl_attribute_index, element.count.try_into().unwrap(), element.gl_type, element.normalized, stride, offset as *const std::ffi::c_void));
                gl_call!(gl::EnableVertexAttribArray(self.gl_attribute_index));
                offset += element.count * VertexBufferElement::get_size_of_type(element.gl_type);

                self.gl_attribute_index += 1;
            }
            let divisors = vbl.get_divisors();
            for divisor in divisors {
                gl_call!(gl::VertexAttribDivisor(divisor.0, divisor.1));
            }
            VertexBuffer::un_bind();
            VertexArray::un_bind();
        }
    }

    pub fn bind(&self)  { unsafe { gl_call!(gl::BindVertexArray(self.gl_buffer_id)); }}
    pub fn un_bind()    { unsafe { gl_call!(gl::BindVertexArray(0)); }}
}

impl Drop for VertexArray {
    fn drop(&mut self)  {unsafe {gl_call!(gl::DeleteVertexArrays(1, &self.gl_buffer_id));}}
}

// TODO implement shader buffer
/*
ShaderBuffer::ShaderBuffer(const void* data, size_t size)
{
    gl_call!(glGenBuffers(1, &m_ID));
    gl_call!(glBindBuffer(gl::SHADER_STORAGE_BUFFER, m_ID));
    gl_call!(glBufferData(gl::SHADER_STORAGE_BUFFER, size, data, gl::DYNAMIC_COPY));
    gl_call!(glBindBuffer(gl::SHADER_STORAGE_BUFFER, 0));
}

ShaderBuffer::~ShaderBuffer()
{
    gl_call!(glDeleteBuffers(1, &m_ID));
}

void ShaderBuffer::Bind(unsigned int index) const
{
    gl_call!(glBindBuffer(gl::SHADER_STORAGE_BUFFER, m_ID));
    gl_call!(glBindBufferBase(gl::SHADER_STORAGE_BUFFER, index, m_ID));
}

void ShaderBuffer::UnBind() const
{
    gl_call!(glBindBuffer(gl::SHADER_STORAGE_BUFFER, 0));
}*/