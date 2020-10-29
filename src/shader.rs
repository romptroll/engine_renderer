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

use std::io::prelude::*;
use std::fs::File;

#[derive(Copy, Clone)]
enum ShaderType {
    NONE = -1, VERTEX = 0, FRAGMENT = 1, GEOMETRY = 2
}

pub struct Shader {
    pub gl_buffer_id : u32
}

impl Shader {
    
    pub fn upload_from_name_1i(&self, uniform: &str, v1: i32)							    { self.upload_1i(self.get_uniform_location(uniform), v1); }
    pub fn upload_from_name_2i(&self, uniform: &str, v1: i32, v2: i32)					    { self.upload_2i(self.get_uniform_location(uniform), v1, v2); }
    pub fn upload_from_name_3i(&self, uniform: &str, v1: i32, v2: i32, v3: i32)			    { self.upload_3i(self.get_uniform_location(uniform), v1, v2, v3); }
    pub fn upload_from_name_4i(&self, uniform: &str, v1: i32, v2: i32, v3: i32, v4: i32)	{ self.upload_4i(self.get_uniform_location(uniform), v1, v2, v3, v4); }

    pub fn upload_1i(&self, location: i32, v1: i32) 										{ unsafe { gl_call!(gl::Uniform1i(location, v1)); } }
    pub fn upload_2i(&self, location: i32, v1: i32, v2: i32) 								{ unsafe { gl_call!(gl::Uniform2i(location, v1, v2)); } }
    pub fn upload_3i(&self, location: i32, v1: i32, v2: i32, v3: i32) 						{ unsafe { gl_call!(gl::Uniform3i(location, v1, v2, v3)); } }
    pub fn upload_4i(&self, location: i32, v1: i32, v2: i32, v3: i32, v4: i32) 				{ unsafe { gl_call!(gl::Uniform4i(location, v1, v2, v3, v4)); } }

	pub fn upload_from_name_1f(&self, uniform: &str, v1: f32)                              	{ self.upload_1f(self.get_uniform_location(uniform), v1); }
	pub fn upload_from_name_2f(&self, uniform: &str, v1: f32, v2: f32)                     	{ self.upload_2f(self.get_uniform_location(uniform), v1, v2); }
    pub fn upload_from_name_3f(&self, uniform: &str, v1: f32, v2: f32, v3: f32)            	{ self.upload_3f(self.get_uniform_location(uniform), v1, v2, v3); }
	pub fn upload_from_name_4f(&self, uniform: &str, v1: f32, v2: f32, v3: f32, v4: f32)   	{ self.upload_4f(self.get_uniform_location(uniform), v1, v2, v3, v4); }

    pub fn upload_1f(&self, location: i32, v1: f32)                              			{ unsafe { gl_call!(gl::Uniform1f(location, v1)); } }
    pub fn upload_2f(&self, location: i32, v1: f32, v2: f32)                     			{ unsafe { gl_call!(gl::Uniform2f(location, v1, v2)); } }
    pub fn upload_3f(&self, location: i32, v1: f32, v2: f32, v3: f32)            			{ unsafe { gl_call!(gl::Uniform3f(location, v1, v2, v3)); } }
	pub fn upload_4f(&self, location: i32, v1: f32, v2: f32, v3: f32, v4: f32)   			{ unsafe { gl_call!(gl::Uniform4f(location, v1, v2, v3, v4)); } }
	
	pub fn upload_from_name_3x3f(&self, uniform: &str, v: &[f32; 9]) 	{ self.upload_3x3f(self.get_uniform_location(uniform), v) }
	pub fn upload_from_name_4x4f(&self, uniform: &str, v: &[f32; 16]) 	{ self.upload_4x4f(self.get_uniform_location(uniform), v) }

	pub fn upload_3x3f(&self, location: i32, v: &[f32; 9])  			{ unsafe { gl_call!(gl::UniformMatrix3fv(location, 1, 0 /*FALSE*/, v as *const f32)); } }
	pub fn upload_4x4f(&self, location: i32, v: &[f32; 16]) 			{ unsafe { gl_call!(gl::UniformMatrix4fv(location, 1, 0 /*FALSE*/, v as *const f32)); } }

    pub fn get_uniform_location(&self, uniform: &str) -> i32 {
		unsafe {
			let location;
			gl_call!(location = gl::GetUniformLocation(self.gl_buffer_id, std::ffi::CString::new(uniform).unwrap().into_raw() as *const i8));

			if location == -1 {
				error_log!("SHADER UNIFORM : {} DOES NOT EXIST", uniform);
			}
			location
		}
	}
		
    pub fn get_type_from_name(&self, uniform: &str)		-> u32 { return self.get_type(self.get_uniform_location(uniform)); }

    pub fn get_type(&self, location: i32)	-> u32 {location as u32}

	pub fn compile(shader_source: &str, shader_type: u32) -> u32 {
		unsafe {
			let id : u32;
			gl_call!(id = gl::CreateShader(shader_type));
			//TODO free this function to avoid memory leak
			let source_ptr = std::ffi::CString::new(shader_source).unwrap().into_raw() as *mut i8;
			
			gl_call!(gl::ShaderSource(id, 1, &(source_ptr as *const i8) as *const *const i8, std::mem::zeroed()));
			gl_call!(gl::CompileShader(id));

			let mut result: i32 = 0;
			gl_call!(gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut result as *mut i32));
			if result == gl::FALSE as i32 {
				let mut length: i32 = 0;
				gl_call!(gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length as *mut i32));
				let mut message: std::vec::Vec::<u8> = vec![0; length as usize];
				gl_call!(gl::GetShaderInfoLog(id, length, &mut length as *mut i32, message.as_mut_ptr() as *mut i8));
				gl_call!(gl::DeleteShader(id));

				match shader_type {
					gl::VERTEX_SHADER 	=> error_log!("Failed to compile VERTEX SHADER"		),
					gl::GEOMETRY_SHADER => error_log!("Failed to compile GEOMETRY SHADER"	),
					gl::FRAGMENT_SHADER => error_log!("Failed to compile FRAGMENT SHADER"	),
					_ =>				   error_log!("Falied to compile SHADER"			),
				}

				error_log!("{}", std::string::String::from_utf8(message).unwrap());
				error_log!("Shader source:\n{}", shader_source);
				return 0;
			}
			id
		}
	}
	
    pub fn parse(shader_source: &str) -> std::vec::Vec::<std::string::String> {
		let mut shaders = vec!(std::string::String::new(), std::string::String::new(), std::string::String::new());
		let mut shader_type = ShaderType::NONE;
		let lines = shader_source.lines();
		for line in lines {
			if line.find("#shader").is_some() {
				if line.find("vertex").is_some() {
					shader_type = ShaderType::VERTEX;
				}
				else if line.find("fragment").is_some() {
					shader_type = ShaderType::FRAGMENT;
				}
				else if line.find("geometry").is_some() {
					shader_type = ShaderType::GEOMETRY;
				}
			}
			else { match shader_type {
				ShaderType::NONE => {},
				shader_type => {
					shaders[shader_type as usize].push_str(&format!("{}\n", line));
				}
			}}
		}
		shaders
	}
    
    pub fn load_file(file_path: &str) -> std::string::String {
        let mut file = File::open(file_path).expect(&format!("Unable to open shader file: {}", file_path));
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(&format!("Unable to read shader file: {}", file_path));
        contents
	}

	pub fn from_source(source: &str) -> Shader {
		unsafe {
			//core::logger::info(source);
			let shader_sources = Shader::parse(source);

			//core::logger::info("DONE");

			let mut vs: u32 = 0;
			let mut fs: u32 = 0;
			let mut gs: u32 = 0;

			let program;
			gl_call!(program = gl::CreateProgram());

			if shader_sources[ShaderType::VERTEX as usize] != "" {
				vs = Shader::compile(&shader_sources[ShaderType::VERTEX as usize], gl::VERTEX_SHADER);
				gl_call!(gl::AttachShader(program, vs));
			}

			if shader_sources[ShaderType::FRAGMENT as usize] != "" {
				fs = Shader::compile(&shader_sources[ShaderType::FRAGMENT as usize], gl::FRAGMENT_SHADER);
				gl_call!(gl::AttachShader(program, fs));
			}

			if shader_sources[ShaderType::GEOMETRY as usize] != "" 	{
				gs = Shader::compile(&shader_sources[ShaderType::GEOMETRY as usize], gl::GEOMETRY_SHADER);
				gl_call!(gl::AttachShader(program, gs));
			}
			
			gl_call!(gl::LinkProgram(program));
			gl_call!(gl::ValidateProgram(program));

			gl_call!(gl::DeleteShader(vs));
			gl_call!(gl::DeleteShader(fs));
			gl_call!(gl::DeleteShader(gs));
			
			Shader { gl_buffer_id: program }
		}
	}
   
	pub fn from_file(file_path: &str) 	-> Shader { Shader::from_source(&Shader::load_file(file_path)) }
	
	pub fn from_files(file_paths: std::vec::Vec<&str>) -> Shader {
		let mut raw_sources = std::string::String::new();
		for file_path in file_paths {
			raw_sources.push_str(&Shader::load_file(file_path));
		}

		Shader::from_source(&raw_sources)

	}

	pub fn bind(&self) 	{ unsafe { gl_call!(gl::UseProgram(self.gl_buffer_id)); }}
	pub fn un_bind() 	{ unsafe { gl_call!(gl::UseProgram(0)); }}
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl_call!(gl::DeleteProgram(self.gl_buffer_id));
        }
    }
}