extern crate bus;
extern crate crossbeam_channel;
extern crate glfw;
extern crate itertools;

mod core;
mod renderer;
mod systems;

use systems::render::RenderSystem;

use crate::core::engine::EngineBuilder;
use crate::core::system::System;

fn main() {
    let mut engine = EngineBuilder::builder()
        .add_system(System::SampleSystem(core::system::SampleSystem {
            name: "0".to_string(),
        }))
        .add_system(System::RenderSystem(RenderSystem::new()))
        .build();

    loop {
        engine.update()
    }
}
