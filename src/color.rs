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

#[derive(Clone, Copy)]
pub struct Color {
    rgba: u32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color::from((r, g, b, a))
    }
}

impl From<u32> for Color { fn from(rgba: u32) -> Self { Self { rgba } } }
impl From<Color> for u32 { fn from(color: Color) -> Self { color.rgba } }

impl From<f32> for Color { fn from(rgba: f32) -> Self { Self { rgba: unsafe { std::mem::transmute(rgba) } } } }
impl From<Color> for f32 { fn from(color: Color) -> Self { unsafe { std::mem::transmute(color.rgba) } } }

impl From<(f32, f32, f32, f32)> for Color {
    fn from(rgba: (f32, f32, f32, f32)) -> Self {
        let r = (rgba.0 * 255.0) as u32;
        let g = (rgba.1 * 255.0) as u32;
        let b = (rgba.2 * 255.0) as u32;
        let a = (rgba.3 * 255.0) as u32;
        Color::from(a | (b << 8) | (g << 16) | (r << 24))
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    fn from(color: Color) -> Self {
        let r = ((color.rgba >> 24) & 0xFF) as f32 / 255.0;
        let g = ((color.rgba >> 16) & 0xFF) as f32 / 255.0;
        let b = ((color.rgba >> 8 ) & 0xFF) as f32 / 255.0;
        let a = ((color.rgba >> 0 ) & 0xFF) as f32 / 255.0;
        (r, g, b, a)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(rgba: (u8, u8, u8, u8)) -> Self {
        let r = rgba.0 as u32;
        let g = rgba.1 as u32;
        let b = rgba.2 as u32;
        let a = rgba.3 as u32;
        Color::from(a | (b << 8) | (g << 16) | (r << 24))
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    fn from(color: Color) -> Self {
        let r = ((color.rgba >> 24) & 0xFF) as u8;
        let g = ((color.rgba >> 16) & 0xFF) as u8;
        let b = ((color.rgba >> 8 ) & 0xFF) as u8;
        let a = ((color.rgba >> 0 ) & 0xFF) as u8;
        (r, g, b, a)
    }
}