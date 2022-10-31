use std::fmt::{self, Debug};
use std::sync::Arc;


use crate::{core::{
    engine::{EntityManagerRef, SystemEvent},
    system::{SysResult, SystemTrait},
}, renderer::window::{init_window, GlfwWindowContext}};

pub struct RenderSystem {
    window_context: Option<GlfwWindowContext>
}


impl Debug for RenderSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RenderSystem is undebuggable")
    }
}


impl SystemTrait for RenderSystem {
    fn init(&mut self) {
        //Setup the windows

        let window = init_window();
        self.window_context = Some(window);
    }

    fn step(&mut self, time: usize, entities: &EntityManagerRef) -> SysResult<Vec<SystemEvent>> {
        let GlfwWindowContext { window, glfw, events } = self.window_context.as_mut().unwrap();

        if window.should_close() {
            return Ok(vec![SystemEvent::ShutdownEngine]);
        }

        glfw.poll_events();

        for (_, event) in glfw::flush_messages(events) {
            println!("{:?}", event);
        }

        Ok(vec![])
    }
}

//TODO: (teddy) Initialize glfw and opengl

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            window_context: None
        }
    }
}
