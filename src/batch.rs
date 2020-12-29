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

use engine_core::error_log;
use crate::buffer;

pub struct Batch {
    vertex_data: std::vec::Vec::<f32>,
}

impl Batch {
    pub fn new(capacity: usize) -> Batch { Batch { vertex_data: std::vec::Vec::with_capacity(capacity) }}

	pub fn add_vertex_data(&mut self, vertex_data: &[f32]) {
        if self.vertex_data.len() + vertex_data.len() > self.vertex_data.capacity() {
            error_log!("Tried to add vertex data with size[{}] to large for batch with capacity of[{}]", vertex_data.len(), self.vertex_data.capacity() - self.vertex_data.len());
        }
        self.vertex_data.extend_from_slice(vertex_data);
    }

    pub fn get(&mut self, vbl: &buffer::VertexBufferLayout) -> (buffer::VertexArray, buffer::VertexBuffer, u32) {
        let vb = buffer::VertexBuffer::new(&self.vertex_data);
        let mut va = buffer::VertexArray::new();
        va.add_buffer(&vb, &vbl);

        let n_vertices = self.vertex_data.len() * 4 / vbl.get_stride() as usize;

        self.vertex_data.clear();

        (va, vb, n_vertices as u32)
    }

    pub fn len(&self) -> usize { self.vertex_data.len() }
    pub fn capacity(&self) -> usize { self.vertex_data.capacity() }

    pub fn to_vec(self) -> Vec<f32> { self.vertex_data }
    pub fn as_vec(&self) -> &Vec<f32> { &self.vertex_data }
    pub fn as_mut_vec(&mut self) -> &mut Vec<f32> { &mut self.vertex_data }
}