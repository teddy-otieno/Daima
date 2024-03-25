use std::{
    ffi::{c_void, CString},
    sync::mpsc::Receiver,
};

use gl::Viewport;
use glfw::{Context, Window, WindowEvent};

pub struct GlfwWindowContext {
    pub glfw: glfw::Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
}

//NOTE: (teddy) If anything goes wrong then its on me
//Just trust me bro
unsafe impl Send for GlfwWindowContext {}
unsafe impl Sync for GlfwWindowContext {}

pub fn init_window() -> GlfwWindowContext {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 800, "Daima", glfw::WindowMode::Windowed)
        .expect("Failed to create Window");

    window.set_key_polling(true);
    window.make_current();
    window.set_pos(300, 100);
    //window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_size_polling(true);

    gl::load_with(|f| window.get_proc_address(f));
    gl::Viewport::load_with(|f| window.get_proc_address(f));

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(message_callback), 0 as *const c_void);
    }

    GlfwWindowContext {
        glfw,
        window,
        events,
    }
}

extern "system" fn message_callback(
    source: gl::types::GLenum,
    e_type: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    user_param: *mut c_void,
) {
    let mut message_buffer = Vec::with_capacity(length.try_into().unwrap());

    unsafe {
        for i in 0..length {
            message_buffer.push(*message.offset(i.try_into().unwrap()))
        }

        let message_bytes: Vec<u8> = message_buffer.into_iter().map(|x| x as u8).collect();
        let c_string = CString::from_vec_unchecked(message_bytes);

        eprintln!(
            "GL CALLBACK: type = {}, severity = {}, {:?}",
            e_type, severity, c_string
        );
    }
}
