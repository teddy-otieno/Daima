extern crate bus;
extern crate crossbeam_channel;
extern crate gl;
extern crate glfw;
extern crate itertools;

mod core;
mod renderer;
mod systems;

use systems::assets::AssetLoaderSystem;
use systems::render::RenderSystem;

use crate::core::engine::EngineBuilder;
use crate::core::level_manager::StarterLevel;
use crate::core::system::System;
use systems::render::RenderSystem;

fn main() {
    let mut engine = EngineBuilder::builder()
        .add_system(System::SampleSystem(core::system::SampleSystem {
            name: "0".to_string(),
        }))
        .add_system(System::RenderSystem(RenderSystem::new()))
        .add_system(System::AssetSystem(AssetLoaderSystem::new()))
        .build();

    loop {
        engine.update()
    }
}
