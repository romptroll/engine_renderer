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



#[macro_use]
pub mod macros;
pub mod log;
pub mod batch;
pub mod buffer;
pub mod font;
pub mod graphics;
pub mod renderer;
pub mod shape;
pub mod texture;
pub mod window;
pub mod shader;
pub mod matrix;
pub mod vector;

#[cfg(test)]
mod tests {
    use crate::shader::Shader;
    use crate::renderer;
    use crate::graphics::*;
    use crate::buffer::*;
    use crate::matrix::*;

    #[test]
    fn multiple_windows() {
        assert!(crate::window::Window::new(600, 400, "win1").is_some());
        assert!(crate::window::Window::new(600, 400, "win2").is_some());
    }

    #[test]
    fn text() {
        let mut win = crate::window::Window::new(600, 400, "Graphics").unwrap();
        win.make_current();
        renderer::init_gl(&mut win);

        let mut gfx = Graphics::new(&mut win);

        let mut m = 0.0;

        while !win.should_close() {

            m += 0.001;

            //gfx.clear_rgba8888(0x00_00_00_FF);

            if m >= 2.0 {
                m -= 2.0;
                gfx.clear_rgba8888(0x00)
            }

            gfx.set_scale(m, m);
            gfx.draw_string("Hmmm ja du det ska man undra sig", -0.5, -0.5);
            gfx.draw_string("yeet", 0.0, 0.0);

            gfx.update();
            gfx.flush();
            win.poll_events();
            win.swap_buffers();
        }
    }

    #[test]
    fn cube() {
        let mut win = crate::window::Window::new(600, 400, "Graphics").unwrap();
        win.make_current();
        renderer::init_gl(&mut win);


        let cube_vertex_data = [
            -1.0, -1.0, -1.0,	//DSW 0
            1.0, -1.0, -1.0,	//DSE 1
            -1.0, 1.0, -1.0,	//USW 2
            1.0, 1.0, -1.0,		//USE 3
            -1.0, -1.0, 1.0,	//DNW 4
            1.0, -1.0, 1.0,		//DNE 5 
            -1.0, 1.0, 1.0,		//UNW 6
            1.0, 1.0, 1.0		//UNE 7
        ];
    
        let cube_index_data = [
            //NORTH
            4, 6, 5, //DNW, UNW, DNE 
            5, 6, 7, //DNE, UNW, UNE, 
            
            //SOUTH
            0, 2, 1, //DSW, USW, DSE
            1, 2, 3, //DSE, USW, USE

            //UP
            2, 6, 3, //USW, UNW, USE
            3, 6, 7, //USE, UNW, UNE

            //DOWN
            0, 4, 1, //DSW, DNW, DSE
            1, 4, 5, //DSE, DNW, DNE

            //EAST
            5, 7, 1, //DNE, UNE, DSE
            1, 7, 3, //DSE, UNE, USE

            //WEST
            4, 6, 0, //DNW, UNW, DSW
            0, 6, 2, //DSW, UNW, USW
        ];

        let mut vertex_buffer_layout = VertexBufferLayout::new();
        vertex_buffer_layout.push_f32(3); //XYZ

        let vertex_buffer = VertexBuffer::new(&cube_vertex_data);
        let mut vertex_array = VertexArray::new();
        vertex_array.add_buffer(&vertex_buffer, &vertex_buffer_layout);
        vertex_array.bind();

        let index_buffer = IndexBuffer::new(&cube_index_data);
        index_buffer.bind();

        let cube_shader = Shader::from_file("res/shaders/cube.glsl");

        let mut gfx = Graphics::new(&mut win);

        let mut m = 0.0;

        unsafe { renderer::std_renderer::enable(renderer::std_renderer::Capability::DepthTest); }

        while !win.should_close() {

            unsafe { renderer::std_renderer::clear(renderer::std_renderer::ClearTarget::Depth); }
            gfx.clear_rgba8888(0x00_00_00_00);

            m += 0.01;

            // Projection Matrix
            let fov = 90.0;
            let aspect_ratio = gfx.frame_height() as f32 / gfx.frame_width() as f32;
            let fov_rad = 1.0 / (fov * 0.5 / 180.0 * std::f32::consts::PI).tan();

            let projection = Mat4x4f::projection(aspect_ratio, fov_rad, 100.0, 0.1);
            let model = Mat4x4f::mult(&Mat4x4f::rotation_y(m), &Mat4x4f::rotation_x(10.5));
            let model = Mat4x4f::mult(&Mat4x4f::translation(0.0, 0.0, 4.0), &model);
            let mvp = Mat4x4f::mult(&projection, &model);

            unsafe {
                cube_shader.bind();
                cube_shader.upload_from_name_4x4f("u_MVP", &mvp.values);
                renderer::std_renderer::draw_elements(renderer::std_renderer::RenderingPrimitive::Triangles, cube_index_data.len() as i32); 
            }

            gfx.update();
            gfx.flush();
            win.poll_events();
            win.swap_buffers();
        }
    }
}