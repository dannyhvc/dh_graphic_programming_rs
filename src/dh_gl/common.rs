/// functions common to quickly setup common opengl vertex buffers and shader compilation.
extern crate gl;
extern crate glfw;

use self::glfw::{Action, Key};
use gl::types::*;
use glfw::Context;
use glfw::Glfw;
use glfw::Window;
use glfw::WindowEvent;
use std::ffi::CString;
use std::fs;
use std::ptr;
use std::sync::mpsc::Receiver;

pub fn dhgl_read_shader_file_into_string(vs_fname: &str, fs_fname: &str) -> (String, String) {
    let vshader = fs::read_to_string(vs_fname);
    let fshader = fs::read_to_string(fs_fname);
    return match (vshader, fshader) {
        (Ok(vshader), Ok(fshader)) => (vshader, fshader),
        (Err(_e), Ok(_fs)) => panic!("Failed to read shader -> {}", vs_fname),
        (Ok(_vs), Err(_e)) => panic!("Failed to read shader -> {}", fs_fname),
        _ => panic!("Failed to read all shader files"),
    };
}

pub unsafe fn dhgl_compile_and_attach_shaders(
    shader_program: u32,
    vs_src: u32,
    fs_src: u32,
    shader_strings: (String, String),
) {
    let v_shader_src_ptr = CString::new(shader_strings.0.as_bytes()).unwrap();
    gl::ShaderSource(vs_src, 1, &v_shader_src_ptr.as_ptr(), ptr::null());
    gl::CompileShader(vs_src);

    /*
    check for shader compile errors for the vertex shader
    */
    let mut success = gl::FALSE as GLint;
    let mut infoLog = Vec::with_capacity(512);
    infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
    gl::GetShaderiv(vs_src, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
            vs_src,
            512,
            ptr::null_mut(),
            infoLog.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
            std::str::from_utf8(&infoLog).unwrap()
        );
    }

    let f_shader_src_ptr = CString::new(shader_strings.1.as_bytes()).unwrap();
    gl::ShaderSource(fs_src, 1, &f_shader_src_ptr.as_ptr(), ptr::null());
    gl::CompileShader(fs_src);


    dhgl_link_shaders(shader_program,&(vs_src, fs_src));

    // deleting the already linked shader.
    gl::DeleteShader(vs_src);
    gl::DeleteShader(fs_src);
}

pub unsafe fn check_shader_compile_error(shadr_src: u32, mut status: i32, info_log: &mut Vec<u8>) {
    /*
    check for shader compile errors for fragment shader
    */
    gl::GetShaderiv(shadr_src, gl::COMPILE_STATUS, &mut status);
    if status != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
            shadr_src,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
            std::str::from_utf8(&info_log).unwrap()
        );
    }
}

pub unsafe fn dhgl_link_shaders(shader_program: u32, shaders_src: &(u32, u32)) {
    gl::AttachShader(shader_program, shaders_src.0);
    gl::AttachShader(shader_program, shaders_src.1);
    gl::LinkProgram(shader_program);
}

pub fn dhgl_create_window(
    init: &mut Glfw,
    width: u32,
    height: u32,
    title: &str,
    mode: glfw::WindowMode,
) -> (Window, Receiver<(f64, WindowEvent)>) {
    init.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    init.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    init.create_window(width, height, title, mode)
        .expect("Failed to create GLFW window")
}

pub fn dhgl_window_common_attrib_setter(window: &mut Window) {
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
}

// NOTE: not the same version as in common.rs!
pub fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
