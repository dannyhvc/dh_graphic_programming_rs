#[allow(dead_code)]
extern crate gl;
extern crate glfw;
use crate::dh_gl::common;

#[allow(unused_imports)]
use self::glfw::{Action, Context, Glfw, Key, Window, WindowEvent};
use std::ffi::CString;
use std::ptr;
use std::sync::mpsc::Receiver;

pub struct QuickGlr {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    shader_program: u32,
    vao: u32,
    vbo: u32,
}

impl QuickGlr {
    pub fn new(width: u32, height: u32, title: &str, mode: glfw::WindowMode) -> Self {
        // glfw: initialize and configure
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) =
            common::dhgl_create_window(&mut glfw, width, height, title, mode);
        common::dhgl_window_common_attrib_setter(&mut window);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // retval
        return unsafe {
            Self {
                glfw,
                window,
                events,
                shader_program: gl::CreateProgram(),
                vao: 0_u32,
                vbo: 0_u32,
            }
        };
    } // new

    pub fn create_and_bind_shaders(&mut self, vs_str: &str, fs_str: &str) {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            common::dhgl_compile_and_attach_shaders(
                self.shader_program,
                vertex_shader,
                frag_shader,
                common::dhgl_read_shader_file_into_string(vs_str, fs_str),
            );
        }
    }
}
