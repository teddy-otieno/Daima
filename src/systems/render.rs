use std::ffi::{c_void, CString};
use std::fmt::{self, Debug};
use std::ptr::null;
use std::sync::Arc;

use glfw::Context;

use crate::core::engine::Engine;
use crate::{
    core::{
        engine::{EntityManagerRef, SystemEvent},
        system::{SysResult, SystemTrait},
    },
    renderer::window::{init_window, GlfwWindowContext},
};

pub struct RenderSystem {
    window_context: Option<GlfwWindowContext>,
    shader_program: Option<u32>,
    vao: Option<u32>,
}

impl Debug for RenderSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("{:?} {:?}", self.vao, self.shader_program).as_str())
    }
}

impl SystemTrait for RenderSystem {
    fn init(&mut self) {
        //Setup the windows

        let window = init_window();
        self.window_context = Some(window);

        unsafe {
            self.compile_shaders();
            self.initialize_render_objects();
        }
    }

    fn step(
        &mut self,
        time: usize,
        entities: &EntityManagerRef,
        engine: &Engine,
    ) -> SysResult<Vec<SystemEvent>> {
        let GlfwWindowContext {
            window,
            glfw,
            events,
        } = self.window_context.as_mut().unwrap();

        window.swap_buffers();
        glfw.poll_events();

        if window.should_close() {
            return Ok(vec![SystemEvent::ShutdownEngine]);
        }

        let window_events = glfw::flush_messages(events)
            .into_iter()
            .map(|(_i, e)| {
                SystemEvent::EngineEvent(crate::core::engine::GameStateEvent::InputEvent(e))
            })
            .collect::<Vec<SystemEvent>>();

        unsafe {
            self.render_to_window(engine);
        }
        Ok(window_events)
    }
}

//TODO: (teddy) Initialize glfw and opengl

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            window_context: None,
            shader_program: None,
            vao: None,
        }
    }

    unsafe fn initialize_render_objects(&mut self) {
        //Rendering triangle
        let vertices = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0 as f32];

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            ((std::mem::size_of::<f32>() as usize) * vertices.len()) as isize,
            (vertices.as_ptr()) as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            0 as *const i32 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);

        self.vao = Some(vao);
    }

    unsafe fn compile_shaders(&mut self) {
        let vertex_shader_source_string = r#"
            #version 330 core
            layout (location = 0) in vec3 a_pos;

            uniform mat4 mvp;

            out vec3 out_color;

            void main() {
                gl_Position = mvp * vec4(a_pos, 1.0);
                out_color = vec3(abs(a_pos.x), abs(a_pos.y), abs(a_pos.z));
            }
            "#;

        let vertex_shader_source = CString::new(vertex_shader_source_string).unwrap();

        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), null());
        gl::CompileShader(vertex_shader);
        let mut sucess: i32 = 0;
        let mut info_log: Vec<i8> = vec![0; 1028];
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut sucess as *mut i32);

        if sucess == 0 {
            gl::GetShaderInfoLog(
                vertex_shader,
                1028,
                null::<i32>() as *mut i32,
                info_log.as_mut_ptr(),
            );

            let message = info_log
                .iter()
                .filter(|s| **s != 0)
                .map(|s| *s as u8)
                .collect();
            eprintln!("{}", String::from_utf8(message).unwrap());
        }

        let fragment_shader_source_string = r#"
            #version 330 core
            out vec4 FragColor;
            in vec3 out_color;

            void main() {
                FragColor = vec4(out_color, 1.0f);
            }
            "#;

        let fragment_shader_source = CString::new(fragment_shader_source_string).unwrap();

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), null());
        gl::CompileShader(fragment_shader);
        let mut sucess: i32 = 0;
        let mut info_log: Vec<i8> = vec![0; 1028];
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut sucess as *mut i32);

        if sucess == 0 {
            gl::GetShaderInfoLog(
                fragment_shader,
                1028,
                null::<i32>() as *mut i32,
                info_log.as_mut_ptr(),
            );

            let message = info_log
                .iter()
                .filter(|s| **s != 0)
                .map(|s| *s as u8)
                .collect();
            eprintln!("{}", String::from_utf8(message).unwrap());
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        self.shader_program = Some(shader_program);
    }

    unsafe fn render_to_window(&mut self, engine: &Engine) {
        let camera = engine.camera.clone();
        let camera_lock = camera.read().unwrap();

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        //Bind view_matrix_here
        let mvp_uniform_name = CString::new("mvp").unwrap();
        let mvp_id =
            gl::GetUniformLocation(self.shader_program.unwrap(), mvp_uniform_name.as_ptr());

        gl::UseProgram(self.shader_program.unwrap());
        let view_matrix = camera_lock.look_matrix();
        gl::UniformMatrix4fv(mvp_id, 1, gl::FALSE, view_matrix.as_ptr());

        gl::BindVertexArray(self.vao.unwrap());
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}
