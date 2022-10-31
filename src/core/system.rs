use crate::systems::render::RenderSystem;

use super::engine::{Engine, EntityManager, EntityManagerRef, SystemEvent};

pub type SysResult<T> = Result<T, SystemError>;

pub trait SystemTrait {
    fn init(&mut self) {}
    fn step(&mut self, time: usize, entities: &EntityManagerRef) -> SysResult<Vec<SystemEvent>>;
}

#[derive(Clone, Debug)]
pub struct SampleSystem {
    pub(crate) name: String,
}
impl SystemTrait for SampleSystem {
    fn step(&mut self, time: usize, entities: &EntityManagerRef) -> SysResult<Vec<SystemEvent>> {
        println!("{:?} stepped", self);
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
}

impl System {
    pub fn init(&mut self) {
        match self {
            System::SampleSystem(sys) => sys.init(),
            System::RenderSystem(sys) => sys.init(),
        }
    }

    pub fn update(
        &mut self,
        time: usize,
        entities: &EntityManagerRef,
    ) -> Result<Vec<SystemEvent>, SystemError> {
        match self {
            System::SampleSystem(sys) => sys.step(time, entities),
            System::RenderSystem(sys) => sys.step(time, entities),
        }
    }
}
