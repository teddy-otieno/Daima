extern crate bus;
extern crate crossbeam_channel;
mod core;

use crate::core::engine::EngineBuilder;
use crate::core::system::System;

fn main() {
    let mut engine = EngineBuilder::builder()
        .add_system(System::SampleSystem(core::system::SampleSystem { name: "0".to_string() }))
        .add_system(System::SampleSystem(core::system::SampleSystem {name: "1".to_string()}))
        .add_system(System::SampleSystem(core::system::SampleSystem {name: "2".to_string()}))
        .add_system(System::SampleSystem(core::system::SampleSystem {name: "3".to_string()}))
        .add_system(System::SampleSystem(core::system::SampleSystem {name: "4".to_string()}))
        .build();

    loop {
        engine.update()
    }
}
