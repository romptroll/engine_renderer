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
use std::rc::Rc;

use engine_core::error_log;
use image::{GenericImageView, save_buffer};

pub struct Texture {
    gl_texture_id : u32,
    width : u32,
    height : u32,
}

impl Texture {
	pub fn new(width: u32, height: u32, buffer: &[u8]) -> Rc<Texture> {
		unsafe {
			let mut texture_id : u32 = 0;
			
			gl_call!(gl::GenTextures(1, &mut texture_id));
			gl_call!(gl::BindTexture(gl::TEXTURE_2D, texture_id));

			gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST 		as i32));
			gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST 		as i32));
			gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE 	as i32));
			gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE 	as i32));

			gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, width as i32, height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, buffer.as_ptr() as *const std::ffi::c_void));

			gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));

			Rc::new(Texture { gl_texture_id: texture_id, width: width, height: height })
		}
	}

	pub fn from_color(width: u32, height: u32, color: u32) -> Rc<Texture> {
		let mut pixels = vec!(0u8; (width * height * 4) as usize);
		for i in 0..(pixels.len() / 4) {
			pixels[i*4+0] 	= 	((color & 0xFF_00_00_00) >> 24) 	as u8;
			pixels[i*4+1] 	= 	((color & 0x00_FF_00_00) >> 16) 	as u8;
			pixels[i*4+2] 	= 	((color & 0x00_00_FF_00) >> 8) 		as u8;
			pixels[i*4+3] 	= 	((color & 0x00_00_00_FF) >> 0) 		as u8;
		}
		Texture::new(width, height, &pixels)
	}

	pub fn from_color_vec(width: u32, height: u32, color: (f32, f32, f32, f32)) -> Rc<Texture> {
		let mut pixels = vec!(0u8; (width * height * 4) as usize);
		for i in 0..(pixels.len() / 4) {
			pixels[i*4+0] 	= 	(color.0 * 255.0) 	as u8;
			pixels[i*4+1] 	= 	(color.1 * 255.0) 	as u8;
			pixels[i*4+2] 	= 	(color.2 * 255.0) 	as u8;
			pixels[i*4+3] 	= 	(color.3 * 255.0) 	as u8;
		}
		Texture::new(width, height, &pixels)
	}

    pub fn from_file(file_path: &str) -> Rc<Texture> {
		let img = Image::from_file(file_path);
		Texture::new(img.width(), img.height(), &img.get_buffer())
	}

	pub fn from_image(image: &Image) -> Rc<Texture> { Texture::new(image.width(), image.height(), &image.get_buffer()) }

	pub fn set_pixels_u32(&self, x: u32, y: u32, width: u32, height: u32, pixels: &Vec::<u8>) {
		unsafe {
			gl_call!(gl::TexSubImage2D(gl::TEXTURE_2D, 0, x as i32, y as i32, width as i32, height as i32, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr() as *const std::ffi::c_void));
		}
	}

	pub fn set_pixels(&self, x: f32, y: f32, width: f32, height: f32, pixels: &Vec::<u8>) {
		let x = (x * self.width() as f32) as u32;
		let y = (y * self.height() as f32) as u32;
		let width = (width * self.width() as f32) as u32;
		let height = (height * self.width() as f32) as u32;
		unsafe {
			gl_call!(gl::TexSubImage2D(gl::TEXTURE_2D, 0, x as i32, y as i32, width as i32, height as i32, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr() as *const std::ffi::c_void));
		}
	}
    
    pub fn bind(&self, slot: u32) {
        unsafe {
		    gl_call!(gl::ActiveTexture(gl::TEXTURE0 + slot));
		    gl_call!(gl::Enable(gl::TEXTURE_2D));
            gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.gl_texture_id));
        }
	}
    
	pub fn un_bind() {
        unsafe {
            gl_call!(gl::Disable(gl::TEXTURE_2D));
            gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
        }
    }
    
    pub fn width(&self) 	-> u32 { self.width 	}
    pub fn height(&self) 	-> u32 { self.height 	}
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl_call!(gl::DeleteTextures(1, &self.gl_texture_id));
        }
    }
}

#[derive(Clone)]
pub struct TextureRegion {
	texture: std::rc::Weak::<Texture>,
	pub x: u32,
	pub y: u32,
	pub width: u32,
	pub height: u32,
}

impl TextureRegion {
	pub fn new(x: u32, y: u32, width: u32, height: u32, texture: &Rc<Texture>) -> TextureRegion {
		TextureRegion {
			texture: Rc::downgrade(texture),
			x: x,
			y: y,
			width: width,
			height: height,
		}
	}

	pub fn new_norm(x: f32, y: f32, width: f32, height: f32, texture: &Rc<Texture>) -> TextureRegion {
		TextureRegion {
			x: (x * texture.width() as f32) as u32,
			y: (y * texture.height() as f32) as u32,
			width: (width * texture.width() as f32) as u32,
			height: (height * texture.height() as f32) as u32,
			texture: Rc::downgrade(texture),
		}
	}

	pub fn new_whole(texture: &Rc<Texture>) -> 	TextureRegion { TextureRegion::new_norm(0.0, 0.0, 1.0, 1.0, texture) }

	pub fn new_invalid() -> TextureRegion {
		TextureRegion {
			texture: std::rc::Weak::new(),
			x: 0,
			y: 0,
			width: 0,
			height: 0,
		}
	}

	pub fn set_pixels_u32(&mut self, x: u32, y: u32, width: u32, height: u32, pixels: &Vec::<u8>) {
		let strong = self.texture.upgrade();
		match strong {
			Some(texture) => {
				texture.set_pixels_u32(x+self.x, y+self.y, width, height, pixels);
			},
			None => {
				error_log!("Tried to set pixels of None texture!");
			}
		}
	}

	pub fn set_pixels(&mut self, x: f32, y: f32, width: f32, height: f32, pixels: &Vec::<u8>) {
		let strong = self.texture.upgrade();
		match strong {
			Some(texture) => {
				let self_x = self.x as f32 / texture.width() as f32;
				let self_y = self.y as f32 / texture.width() as f32;
				texture.set_pixels(x+self_x, y+self_y, width, height, pixels);
			},
			None => {
				error_log!("Tried to set pixels of None texture!");
			}
		}
	}

	pub fn bind(&mut self, slot: u32) {
		let strong = self.texture.upgrade();
		match strong {
			Some(texture) => {
				texture.bind(slot);
			},
			None => {
				error_log!("Tried to bind None texture!");
			}
		}
	}

	pub fn un_bind() { Texture::un_bind(); }

	pub fn is_valid(&self) -> bool {
		let strong = self.texture.upgrade();
		match strong {
			Some(_) => true,
			None => false,
		}
	}

	pub fn has_same_texture(&self, other: &TextureRegion) -> bool {
		self.texture.ptr_eq(&other.texture)
	}

	pub fn norm_x(&self) -> f32 {
		let strong = self.texture.upgrade();
		match strong {
			Some(texture) => {
				self.x as f32 / texture.width() as f32
			},
			None => {
				error_log!("Tried to normalize x of None texture!");
				0.0
			}
		}
	}

	pub fn norm_y(&self) -> f32 {
		let strong = self.texture.upgrade();
		match strong {
			Some(texture) => {
				self.y as f32 / texture.height() as f32
			},
			None => {
				error_log!("Tried to normalize y of None texture!");
				0.0
			}
		}
	}

	pub fn norm_width(&self) -> f32 {
		let strong = self.texture.upgrade();
		match strong {
			Some(texture) => {
				self.width as f32 / texture.width() as f32
			},
			None => {
				error_log!("Tried to get width of None texture!");
				0.0
			}
		}
	}

	pub fn norm_height(&self) -> f32 {
		let strong = self.texture.upgrade();
		match strong {
			Some(texture) => {
				self.height as f32 / texture.height() as f32
			},
			None => {
				error_log!("Tried to get height of None texture!");
				0.0
			}
		}
	}

	pub fn norm(&self) -> (f32, f32, f32, f32) {
		let strong = self.texture.upgrade();
		match strong {
			Some(texture) => {
				let x = self.x as f32 / texture.width() as f32;
				let y = self.y as f32 / texture.height() as f32;
				let width = self.width as f32 / texture.width() as f32;
				let height = self.height as f32 / texture.height() as f32;
				(x, y, width, height)
			},
			None => {
				error_log!("Tried to normalize None texture!");
				(0.0, 0.0, 0.0, 0.0)
			}
		}
	}

	pub fn tex_coords(&self) -> (f32, f32, f32, f32) {
		let mut tex_coords = self.norm();
		tex_coords.2 += tex_coords.0;
		tex_coords.3 += tex_coords.1;
		tex_coords
	}
}

pub struct Image {
	width: u32,
	height: u32,
	buffer: std::vec::Vec::<u8>,
}

impl Image {
	pub fn new(width: u32, height: u32, buffer: std::vec::Vec::<u8>) -> Image {
		Image {
			width,
			height,
			buffer,
		}
	}

	pub fn from_file(file_path: &str) -> Image {
		let img = image::open(file_path);
		match img {
			Ok(img) => {
				let width 	= img.width();
				let height 	= img.height();
		
				let img = img.flipv();
				let img = img.into_rgba();
				return Image::new(width, height, img.to_vec())
			},
			Err(e) => {
				error_log!("Failed to load image: {}\n{}", file_path, e);
				return Image::from_color(1, 1, 0xFF_FF_FF_FF);
			}
		}
	}

	pub fn from_color(width: u32, height: u32, color: u32) -> Image {
		let mut pixels = vec!(0u8; (width * height * 4) as usize);
		for i in 0..(pixels.len() / 4) {
			pixels[i*4+0] 	= 	((color & 0xFF_00_00_00) >> 24) 	as u8;
			pixels[i*4+1] 	= 	((color & 0x00_FF_00_00) >> 16) 	as u8;
			pixels[i*4+2] 	= 	((color & 0x00_00_FF_00) >> 8) 		as u8;
			pixels[i*4+3] 	= 	((color & 0x00_00_00_FF) >> 0) 		as u8;
		}
		Image::new(width, height, pixels)
	}

	pub fn to_file(&self, path: &str) {
		let img = self.flip_horizontally();
		let buff = img.buffer;
		save_buffer(path, &buff, self.width, self.height, image::ColorType::Rgba8).unwrap();
	}

	pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Image {
		let mut pixels = std::vec::Vec::with_capacity((width*height*4) as usize);
		for j in 0..height {
			for i in 0..width {
				pixels.push(self.get_r8(x+i, y+j));
				pixels.push(self.get_g8(x+i, y+j));
				pixels.push(self.get_b8(x+i, y+j));
				pixels.push(self.get_a8(x+i, y+j));
			}
		}
		Image::new(width, height, pixels)
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		let mut crop_width = width;
		if self.width < width {
			crop_width = self.width;
		}

		let mut crop_height = height;
		if self.height < height {
			crop_height = self.height;
		}

		let mut resized_image = Image::from_color(width, height, 0x00_00_00_00);
		resized_image.draw(0, 0, self.crop(0, 0, crop_width, crop_height));

		self.width  = resized_image.width;
		self.height = resized_image.height;
		self.buffer = resized_image.buffer;
	}

	pub fn draw(&mut self, x: u32, y: u32, image: Image) {
		for i in 0..image.width {
			for j in 0..image.height {
				//self.set_rgba8(x+i, y+j, image.get_rgba8(i, j))
				self.buffer[(((x+i) + (y+j) * self.width) * 4 + 0) as usize] = image.buffer[((i + j * image.width) * 4 + 0) as usize];
				self.buffer[(((x+i) + (y+j) * self.width) * 4 + 1) as usize] = image.buffer[((i + j * image.width) * 4 + 1) as usize];
				self.buffer[(((x+i) + (y+j) * self.width) * 4 + 2) as usize] = image.buffer[((i + j * image.width) * 4 + 2) as usize];
				self.buffer[(((x+i) + (y+j) * self.width) * 4 + 3) as usize] = image.buffer[((i + j * image.width) * 4 + 3) as usize];
			}
		}
	}

	pub fn flip_horizontally(&self) -> Image {
		let mut buff = Vec::with_capacity((self.width*self.height*4) as usize);

		for i in 0..(self.width*self.height) as usize {
			buff.push(self.buffer[self.buffer.len()-1-i*4-3]);
			buff.push(self.buffer[self.buffer.len()-1-i*4-2]);
			buff.push(self.buffer[self.buffer.len()-1-i*4-1]);
			buff.push(self.buffer[self.buffer.len()-1-i*4-0]);
		}

		Image::new(self.width, self.height, buff)
	}

	pub fn get_rgba8(&self, x: u32, y: u32) -> u32 {
		let mut pixel = 0;
		pixel += (self.buffer[((x+y*self.width) * 4 + 0) as usize] as u32) << 24;
		pixel += (self.buffer[((x+y*self.width) * 4 + 1) as usize] as u32) << 16;
		pixel += (self.buffer[((x+y*self.width) * 4 + 2) as usize] as u32) << 8;
		pixel += (self.buffer[((x+y*self.width) * 4 + 3) as usize] as u32) << 0;
		pixel
	}

	pub fn set_rgba8(&mut self, x: u32, y: u32, color: u32) {
		self.buffer[((x+y*self.width) * 4 + 0) as usize] = ((color & 0xFF_00_00_00) >> 24) as u8;
		self.buffer[((x+y*self.width) * 4 + 1) as usize] = ((color & 0x00_FF_00_00) >> 16) as u8;
		self.buffer[((x+y*self.width) * 4 + 2) as usize] = ((color & 0x00_00_FF_00) >> 8)  as u8;
		self.buffer[((x+y*self.width) * 4 + 3) as usize] = ((color & 0x00_00_00_FF) >> 0)  as u8;
	}

	pub fn get_r8(&self, x: u32, y: u32) -> u8 { self.buffer[((x+y*self.width) * 4 + 0) as usize] }
	pub fn get_g8(&self, x: u32, y: u32) -> u8 { self.buffer[((x+y*self.width) * 4 + 1) as usize] }
	pub fn get_b8(&self, x: u32, y: u32) -> u8 { self.buffer[((x+y*self.width) * 4 + 2) as usize] }
	pub fn get_a8(&self, x: u32, y: u32) -> u8 { self.buffer[((x+y*self.width) * 4 + 3) as usize] }

	pub fn set_r8(&mut self, x: u32, y: u32, color: u8) { self.buffer[((x+y*self.width) * 4 + 0) as usize] = color }
	pub fn set_g8(&mut self, x: u32, y: u32, color: u8) { self.buffer[((x+y*self.width) * 4 + 1) as usize] = color }
	pub fn set_b8(&mut self, x: u32, y: u32, color: u8) { self.buffer[((x+y*self.width) * 4 + 2) as usize] = color }
	pub fn set_a8(&mut self, x: u32, y: u32, color: u8) { self.buffer[((x+y*self.width) * 4 + 3) as usize] = color }

	pub fn get_buffer(&self) -> &std::vec::Vec::<u8> { &self.buffer }

	pub fn width(&self)  -> u32 { self.width  } 
	pub fn height(&self) -> u32 { self.height }
}

type ImagePackNodeID = u32;

pub struct ImagePackNode {
	x: u32,
	y: u32,
	width: u32,
	height: u32,
	image_width: u32,
	image_height: u32,
	available: bool,
	child_1: std::option::Option::<ImagePackNodeID>,
	child_2: std::option::Option::<ImagePackNodeID>,
}

impl ImagePackNode {
	fn new(x: u32, y: u32, width: u32, height: u32) -> ImagePackNode {
		ImagePackNode {
			x,
			y,
			width,
			height,
			image_width: width,
			image_height: height,
			available: true,
			child_1: None,
			child_2: None,
		}
	}

	fn has_space(&self, width: u32, height: u32) -> bool {
		self.width >= width && self.height >= height && self.available
	}
}

pub struct ImagePack {
	bitmap: Image,
	pub nodes: std::vec::Vec::<ImagePackNode>,
	head: ImagePackNodeID,
	locations: std::collections::HashMap::<String, u32>,
}

impl ImagePack {
	pub fn new() -> ImagePack {
		ImagePack {
			bitmap: Image::from_color(1, 1, 0x00_00_00_00),
			nodes: std::vec::Vec::new(),
			head: 0,
			locations: std::collections::HashMap::new(),
		}
	}

	fn split_horz(&mut self, node_id: ImagePackNodeID) {
		let node = &mut self.nodes[node_id as usize];

		let up = ImagePackNode::new(
			node.x,
			node.y+node.height/2,
			node.width,
			node.height/2,
		);

		let down = ImagePackNode::new(
			node.x,
			node.y,
			node.width,
			node.height/2,
		);

		node.available = false;
		self.nodes.push(up);
		self.nodes.push(down);
	}

	fn split_vert(&mut self, node_id: ImagePackNodeID) {
		let node = &mut self.nodes[node_id as usize];

		let right = ImagePackNode::new(
			node.x+node.width/2,
			node.y,
			node.width/2,
			node.height,
		);

		let left = ImagePackNode::new(
			node.x,
			node.y,
			node.width/2,
			node.height,
		);

		node.available = false;
		self.nodes.push(right);
		self.nodes.push(left);
	}

	fn double_horz(&mut self) {
		let down = &self.nodes[self.head as usize];
		let up = ImagePackNode::new(0, down.height, down.width, down.height);

		let mut new_head = ImagePackNode::new(0, 0, down.width, down.height * 2);
		new_head.available = false;

		new_head.child_1 = Some(self.head);
		new_head.child_2 = Some(self.nodes.len() as u32 + 1);

		self.head = self.nodes.len() as u32;

		self.bitmap.resize(new_head.width, new_head.height);

		self.nodes.push(new_head);
		self.nodes.push(up);

	}

	fn double_vert(&mut self) {
		let left = &self.nodes[self.head as usize];
		let right = ImagePackNode::new(left.width, 0, left.width, left.height);

		let mut new_head = ImagePackNode::new(0, 0, left.width * 2, left.height);
		new_head.available = false;

		new_head.child_1 = Some(self.head);
		new_head.child_2 = Some(self.nodes.len() as u32 + 1);

		self.head = self.nodes.len() as u32;

		self.bitmap.resize(new_head.width, new_head.height);

		self.nodes.push(new_head);
		self.nodes.push(right);
	}

	fn has_space(&mut self, width: u32, height: u32) -> bool {
		for node in &mut self.nodes {
			if node.available && node.has_space(width, height) {
				return true;
			}
		}
		false
	}

	fn get_empty_node(&mut self, width: u32, height: u32) -> ImagePackNodeID {
		while !self.has_space(width, height) {
			if self.bitmap.width() <= self.bitmap.height() {
				self.double_vert();
			} else {
				self.double_horz();
			}
		}
		
		let mut smallest_possible_node_id = 0;
		for node in &self.nodes {
			if node.has_space(width, height) {
				break;
			}
			smallest_possible_node_id += 1;
		}

		let mut id = 0;
		for i in 0..self.nodes.len() {
			let node = &self.nodes[i];
			
			if node.has_space(width, height) {
				let other_node = &self.nodes[smallest_possible_node_id];
				if node.width * node.height < other_node.width * other_node.height {
					smallest_possible_node_id = id;
				}
			}
			
			id += 1;
		}

		let mut smallest_node = &mut self.nodes[smallest_possible_node_id];

		while smallest_node.has_space(width * 2, height) || smallest_node.has_space(width, height * 2) {
			if smallest_node.has_space(width * 2, height) {
				self.split_vert(smallest_possible_node_id as ImagePackNodeID);
				smallest_possible_node_id = self.nodes.len()-1;
				smallest_node = &mut self.nodes[smallest_possible_node_id];  
			}
			if smallest_node.has_space(width, height * 2) {
				self.split_horz(smallest_possible_node_id as ImagePackNodeID);
				smallest_possible_node_id = self.nodes.len()-1;
				smallest_node = &mut self.nodes[smallest_possible_node_id];
			}
		}

		self.nodes[smallest_possible_node_id].available = false;
		self.nodes[smallest_possible_node_id].image_width = width;
		self.nodes[smallest_possible_node_id].image_height = height;
		return smallest_possible_node_id as ImagePackNodeID;
	}

	pub fn add_image(&mut self, image_name: &str, image: Image) {
		if self.nodes.len() == 0 {
			self.nodes.push(ImagePackNode::new(0, 0, image.width(), image.height()));
			self.bitmap.resize(image.width(), image.height());
		}

		let id = self.get_empty_node(image.width(), image.height());
		let node = &self.nodes[id as usize];
		
		self.locations.insert(image_name.to_string(), id);
		self.bitmap.draw(node.x, node.y, image);
	}

	pub fn get_image(&mut self, image_name: &str) -> Image {
		let id = self.locations.get(image_name).unwrap();
		let node = &self.nodes[(*id) as usize];
		self.bitmap.crop(node.x, node.y, node.width, node.height)
	}

	pub fn get_bitmap(&self) -> &Image { &self.bitmap }
}

pub struct TextureAtlas {
	texture: Rc::<Texture>,
	textures: std::collections::HashMap::<String, TextureRegion>,
}

impl TextureAtlas {
	pub fn new(texture: Texture) -> TextureAtlas {
		TextureAtlas {
			texture: std::rc::Rc::new(texture),
			textures: std::collections::HashMap::new(),
		}
	}

	pub fn from_image_pack(image_pack: &ImagePack) -> TextureAtlas {
		let texture = Texture::from_image(&image_pack.bitmap);
		let mut textures = std::collections::HashMap::new();

		for (texture_name, node_id) in image_pack.locations.iter() {
			let node = &image_pack.nodes[*node_id as usize];
			let texture_region = TextureRegion::new(node.x, node.y, node.image_width, node.image_height, &texture);
			textures.insert(texture_name.to_string(), texture_region);
		}

		TextureAtlas {
			texture,
			textures,
		}
	}

	pub fn texture(self) -> std::rc::Rc::<Texture> { self.texture }

	pub fn get(&self, texture_name: &str) -> TextureRegion { self.textures.get(texture_name).unwrap().clone() }

	pub fn set(&mut self, texture_name: &str, region: TextureRegion) {
		if !std::rc::Rc::ptr_eq(&region.texture.upgrade().unwrap(), &self.texture) {
			error_log!("You can't set a texture region inside a texture atlas which points to a different texture!");
			return;
		}
		self.textures.insert(texture_name.to_string(), region);
	}

	pub fn bind(&mut self) 	{ self.texture.bind(0); }
	pub fn un_bind() 		{ Texture::un_bind();   }
}