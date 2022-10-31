use std::sync::mpsc::Receiver;

use glfw::{Context, Window, WindowEvent};


pub struct GlfwWindowContext {
	pub glfw: glfw::Glfw,
	pub window: Window,
	pub events: Receiver<(f64, WindowEvent)>
}


//NOTE: (teddy) If anything goes wrong then its on me
//Just trust me bro
unsafe impl Send for GlfwWindowContext {}
unsafe impl Sync for GlfwWindowContext {}

pub fn init_window() -> GlfwWindowContext {
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	let (mut window, events) = glfw.create_window(300, 400, "Daima", glfw::WindowMode::Windowed).expect("Failed to create Window");

	window.set_key_polling(true);
	window.make_current();

	GlfwWindowContext { glfw, window, events }
}