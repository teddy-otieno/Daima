use crate::systems::{assets::AssetLoaderSystem, render::RenderSystem};

use super::engine::{Engine, EntityManager, EntityManagerRef, SystemEvent};

pub type SysResult<T> = Result<T, SystemError>;

pub trait SystemTrait {
    fn init(&mut self) {}
    fn step(
        &mut self,
        time: usize,
        entities: &EntityManagerRef,
        engine: &Engine,
    ) -> SysResult<Vec<SystemEvent>>;
}

#[derive(Clone, Debug)]
pub struct SampleSystem {
    pub(crate) name: String,
}
impl SystemTrait for SampleSystem {
    fn step(
        &mut self,
        time: usize,
        entities: &EntityManagerRef,
        engine: &Engine,
    ) -> SysResult<Vec<SystemEvent>> {
        Ok(vec![])
    }
}

/// Static dispatch for systems

#[derive(Debug, Clone)]
pub struct SystemError {
    name: String,
    description: String,
}

#[derive(Debug)]
pub enum System {
    SampleSystem(SampleSystem),
    RenderSystem(RenderSystem),
    AssetSystem(AssetLoaderSystem),
}

impl System {
    pub fn init(&mut self) {
        match self {
            System::SampleSystem(sys) => sys.init(),
            System::RenderSystem(sys) => sys.init(),
            System::AssetSystem(sys) => sys.init(),
        }
    }

    pub fn update(
        &mut self,
        time: usize,
        entities: &EntityManagerRef,
        engine: &Engine,
    ) -> Result<Vec<SystemEvent>, SystemError> {
        match self {
            System::SampleSystem(sys) => sys.step(time, entities, engine),
            System::RenderSystem(sys) => sys.step(time, entities, engine),
            System::AssetSystem(sys) => sys.step(time, entities, engine),
        }
    }
}
