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

use super::buffer;

pub fn new_rect(x: f32, y: f32, width: f32, height: f32) -> (buffer::VertexArray, buffer::VertexBuffer, buffer::VertexBufferLayout) {
    let mut va = buffer::VertexArray::new();
    let vb = buffer::VertexBuffer::new(&[
        x, y,                   //(0, 0)
        x + width, y,           //(1, 0)
        x, y + height,          //(0, 1)

        x + width, y,           //(1, 0)
        x + width, y + height,  //(1, 1)
        x, y + height,          //(0, 1)
    ]);
    let mut vbl = buffer::VertexBufferLayout::new();
    vbl.push_f32(2);
    va.add_buffer(&vb, &vbl);
    (va, vb, vbl)
}

pub fn new_quad(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) -> (buffer::VertexArray, buffer::VertexBuffer, buffer::VertexBufferLayout) {
    let mut va = buffer::VertexArray::new();
    let vb = buffer::VertexBuffer::new(&[
        x1, y1, //(0, 0)
        x2, y2, //(1, 0)
        x3, y3, //(0, 1)

        x2, y2, //(1, 0)
        x4, y4, //(1, 1)
        x3, y3, //(0, 1)
    ]);
    let mut vbl = buffer::VertexBufferLayout::new();
    vbl.push_f32(2);
    va.add_buffer(&vb, &vbl);
    (va, vb, vbl)
}

pub fn new_sprite(x: f32, y: f32, width: f32, height: f32) -> (buffer::VertexArray, buffer::VertexBuffer, buffer::VertexBufferLayout, buffer::IndexBuffer) {
    let mut va = buffer::VertexArray::new();
    let vb = buffer::VertexBuffer::new(&[
        x, y,                   0.0, 0.0, //(0, 0)
        x + width, y,           1.0, 0.0, //(1, 0)
        x, y + height,          0.0, 1.0, //(0, 1)
        x + width, y + height,  1.0, 1.0, //(1, 1)
    ]);
    let mut vbl = buffer::VertexBufferLayout::new();
    vbl.push_f32(2);
    vbl.push_f32(2);
    va.add_buffer(&vb, &vbl);

    let ib = buffer::IndexBuffer::new(&[
        0, 1, 2,
        2, 1, 3
    ]);
    (va, vb, vbl, ib)
}

pub fn new_line(x1: f32, y1: f32, x2: f32, y2: f32) -> (buffer::VertexArray, buffer::VertexBuffer, buffer::VertexBufferLayout) {
    let mut va = buffer::VertexArray::new();
    let vb = buffer::VertexBuffer::new(&[
        x1, y1,                   
        x2, y2,
    ]);
    let mut vbl = buffer::VertexBufferLayout::new();
    vbl.push_f32(2);
    va.add_buffer(&vb, &vbl);
    (va, vb, vbl)
}
