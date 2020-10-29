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

use std::vec::Vec;

use glfw::{Context};

use bus::{Bus};

pub type Action = glfw::Action;
pub type Key    = glfw::Key;
pub type Mouse  = glfw::MouseButton;

pub static mut GLFW: std::option::Option::<glfw::Glfw> = std::option::Option::None;

struct WindowEventSenders {
    key            : Vec<bus::Bus::<(Key, Action)>>,
    mouse          : Vec<bus::Bus::<(Mouse, Action)>>,
    mouse_move     : Vec<bus::Bus::<(f32, f32)>>,
    frame_buffer   : Vec<bus::Bus::<(u32, u32)>>,
}

impl WindowEventSenders {
    fn new() -> WindowEventSenders {
        WindowEventSenders {
            key         : Vec::new(),
            mouse       : Vec::new(),
            mouse_move  : Vec::new(),
            frame_buffer: Vec::new(),
        }
    }
}

pub struct Window {
    pub glfw_window             : glfw::Window,
    glfw_events                 : std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    /*key_event_sender            : bus::Bus::<(Key, Action)>,
    mouse_event_sender          : bus::Bus::<(Mouse, Action)>,
    mouse_move_event_sender     : bus::Bus::<(f32, f32)>,
    frame_buffer_event_sender   : bus::Bus::<(u32, u32)>,*/
    event_senders               : WindowEventSenders,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> std::option::Option::<Window> {
        unsafe {
            if !GLFW.is_some() {
                GLFW = std::option::Option::Some(glfw::init(glfw::FAIL_ON_ERRORS).unwrap());
            }
            
            match &mut GLFW {
                Some(t) => {
                    let mode = glfw::WindowHint::DoubleBuffer(false);
                    t.window_hint(mode);

                    let (window, events) = t.create_window(width, height, title, glfw::WindowMode::Windowed).expect("Failed to create new window!");

                    let window = Window {
                        glfw_window                 : window,
                        glfw_events                 : events,
                        /*key_event_sender            : bus::Bus::new(512),
                        mouse_event_sender          : bus::Bus::new(512),
                        mouse_move_event_sender     : bus::Bus::new(512),
                        frame_buffer_event_sender   : bus::Bus::new(512),*/
                        event_senders               : WindowEventSenders::new(),
                    };
                    std::option::Option::Some(window)
                }
                None => None
            }
	    }
    }

    pub fn set_size(&mut self, width: u32, height: u32) { self.glfw_window.set_size(width as i32, height as i32); }

    pub fn swap_buffers(&mut self)  { self.glfw_window.swap_buffers() }

    pub fn poll_events(&mut self) { 
        unsafe { glfw::ffi::glfwPollEvents() }
        
        for (_, event) in glfw::flush_messages(&self.glfw_events) {
            //println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => {
                    if key as usize >= 512 {
                        error_log!("Key was outside of range 0-512: {}", key as isize);
                    }
                    else {
                        for sender in &mut self.event_senders.key {
                            match sender.try_broadcast((key, action)) {
                                Ok(_) => {},
                                Err(_) => {}//{core::logger::error("Window failed to notify children of a key press/release change!")},
                            }
                        }
                    }
                },
                glfw::WindowEvent::MouseButton(mouse, action, _) => {
                    for sender in &mut self.event_senders.mouse {
                        match sender.try_broadcast((mouse, action)) {
                            Ok(_) => {},
                            Err(_) => {}//{core::logger::error("Window failed to notify children of a mouse button press/release change!")},
                        }
                    }
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = ((x as f32 / self.get_width() as f32) - 0.5) * 2.0;
                    let y = ((y as f32 / self.get_height() as f32) - 0.5) * -2.0;
                    for sender in &mut self.event_senders.mouse_move {
                        match sender.try_broadcast((x, y)) {
                            Ok(_) => {},
                            Err(_) => {}//{core::logger::error("Window failed to notify children of a cursor pos change!")},
                        }
                    }
                },
                glfw::WindowEvent::FramebufferSize(w, h) => {
                    unsafe { gl_call!(gl::Viewport(0, 0, w, h)); };
                    for sender in &mut self.event_senders.frame_buffer {
                        match sender.try_broadcast((w as u32, h as u32)) {
                            Ok(_) => {},
                            Err(_) => {} //{core::logger::error("Window failed to notify children of a framebuffer size change!")},
                        }
                    }
                },
                _ => { 
                    //engine::logger::info("dd");
                },
            }
        }
    }

    pub fn set_title(&mut self, title: &str) { self.glfw_window.set_title(title) }

    pub fn get_size(&self)      -> (i32, i32) { self.glfw_window.get_size() }
    pub fn get_width(&self)     -> i32 { self.get_size().0 }
    pub fn get_height(&self)    -> i32 { self.get_size().1 }

    pub fn make_current(&mut self) {
        self.glfw_window.make_current();
        self.glfw_window.set_key_polling(true);
        self.glfw_window.set_mouse_button_polling(true);
        self.glfw_window.set_cursor_pos_polling(true);
        self.glfw_window.set_framebuffer_size_polling(true);
    }

    pub fn should_close(&self) -> bool { self.glfw_window.should_close() }

    pub fn create_key_listener(&mut self) -> bus::BusReader::<(Key, Action)> { 
        let mut bus = Bus::new(64);
        let reader = bus.add_rx();
        self.event_senders.key.push(bus);
        reader
    }

    pub fn create_mouse_listener(&mut self) -> bus::BusReader::<(Mouse, Action)> {
        let mut bus = Bus::new(64);
        let reader = bus.add_rx();
        self.event_senders.mouse.push(bus);
        reader        
    }

    pub fn create_mouse_move_listener(&mut self) -> bus::BusReader::<(f32, f32)> {
        let mut bus = Bus::new(64);
        let reader = bus.add_rx();
        self.event_senders.mouse_move.push(bus);
        reader   
    }

    pub fn create_frame_buffer_listener(&mut self) -> bus::BusReader::<(u32, u32)> {
        let mut bus = Bus::new(64);
        let reader = bus.add_rx();
        self.event_senders.frame_buffer.push(bus);
        reader
    }
}
