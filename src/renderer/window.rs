use std::sync::mpsc::Receiver;

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

    let (height, width) = (400, 300);

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(width, height, "Daima", glfw::WindowMode::Windowed)
        .expect("Failed to create Window");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    gl::Viewport::load_with(|s| window.get_proc_address(s));

    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
    }

    GlfwWindowContext {
        glfw,
        window,
        events,
    }
}
