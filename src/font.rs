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

use crate::texture;
use freetype::Library;
 
pub struct Font {
    pub atlas: std::rc::Rc::<texture::Texture>,
    glyphs: std::collections::HashMap::<char, Glyph>,
    width: u32,
}

#[derive(Clone)]
pub struct Glyph {
    size: (f32, f32),
    bearing: (f32, f32),
    advance: f32,
    texture: texture::TextureRegion,
}
    
impl Font {
    pub fn new(filepath: &str, size: u32) -> Font {
        let font_height = size * 2;
        let font_width = size;

        // Init the library
        let lib = Library::init().unwrap();
        // Load a font face
        let face = lib.new_face(filepath, 0).unwrap();
        // Set the font size
        face.set_pixel_sizes(font_width, font_height).unwrap();
        // Load a character
        
        let mut glyphs = std::collections::HashMap::new();

        let mut image_pack = texture::ImagePack::new();

        for i in 33..127 {
            face.load_char(i as usize as usize, freetype::face::LoadFlag::RENDER).unwrap();
            // Get the glyph instance
            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            
            let width = bitmap.width();
            let height = bitmap.rows();

            let bearing_x = glyph.bitmap_left() as f32;
            let bearing_y = glyph.bitmap_top() as f32;

            let advance = (glyph.advance().x >> 6) as f32;

            let mut bitmap_converted = texture::Image::from_color(width as u32, height as u32, 0x00_00_00_00);

            for x in 0..width {
                for y in 0..height {
                    let pixel = bitmap.buffer()[(x + y * width) as usize];
                    bitmap_converted.set_r8(x as u32, (height - y - 1) as u32, pixel);
                    bitmap_converted.set_g8(x as u32, (height - y - 1) as u32, pixel);
                    bitmap_converted.set_b8(x as u32, (height - y - 1) as u32, pixel);
                    bitmap_converted.set_a8(x as u32, (height - y - 1) as u32, pixel);
                }
            }

            let size = (width as f32, height as f32);
            let bearing = (bearing_x as f32, bearing_y as f32);

            let glyph = Glyph {
                size,
                bearing,
                advance,
                texture: texture::TextureRegion::new_invalid(),
            };

            glyphs.insert(i as u8 as char, glyph);

            image_pack.add_image(&(i as u8 as char).to_string(), bitmap_converted);
        }
        
        let atlas = texture::TextureAtlas::from_image_pack(&image_pack);
        
        for (k, v) in glyphs.iter_mut() {
            v.texture = atlas.get(&k.to_string());
        }

        Font {
            atlas: atlas.texture(),
            glyphs,
            width: size,
        }
    }

    pub fn new_invalid() -> Font {
        Font {
            atlas: texture::Texture::from_color(1, 1, 0xFF_FF_FF_FF),
            glyphs: std::collections::HashMap::new(),
            width: 1,
        }
    }

    pub fn text_width(&self, text: &str) -> f32 {
        let mut text_width = 0.0;

        let no_glyph_advance = match self.glyph('?') {
            Some(g) => g.advance,
            None => self.width as f32,
        };

        for (i, c) in text.chars().enumerate() {

            if i == 0 {
                match self.glyph(c) {
                    Some(g) => {
                        text_width += g.size().0 + g.bearing().0;
                    }
                    None => {
                        text_width += no_glyph_advance;
                    }
                }
            } else if i == text.len() - 1 {
                match &self.glyph(c) {
                    Some(g) => {
                        text_width += g.advance() - g.bearing().0;
                    }
                    None => {
                        text_width += no_glyph_advance;
                    }
                }
            } else {
                match &self.glyph(c) {
                    Some(g) => {
                        text_width += g.advance();
                    }
                    None => {
                        text_width += no_glyph_advance;
                    }
                }
            }
        }

        
        text_width
    }

    pub fn glyph(&self, glyph: char)     -> Option<&Glyph>  {  self.glyphs.get(&glyph)   }
    pub fn width(&self)                  -> u32             {  self.width                }
    pub fn height(&self)                 -> u32             {  self.width*2              }
    
}

impl Glyph {
    pub fn bind(&mut self) { self.texture.bind(0);              }
    pub fn un_bind()       { texture::TextureRegion::un_bind(); }

    pub fn texture(&self)   -> texture::TextureRegion   { self.texture.clone()  }
    pub fn size(&self)      -> (f32, f32)               { self.size             }
    pub fn bearing(&self)   -> (f32, f32)               { self.bearing          }
    pub fn advance(&self)   -> f32                      { self.advance          }        
}