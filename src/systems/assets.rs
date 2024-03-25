use crate::core::{
    engine::{Engine, EntityManagerRef},
    system::{SysResult, SystemTrait},
};

#[derive(Debug)]
pub struct AssetLoaderSystem {}

impl SystemTrait for AssetLoaderSystem {
    fn step(
        &mut self,
        time: usize,
        entities: &EntityManagerRef,
        engine: &Engine,
    ) -> SysResult<Vec<crate::core::engine::SystemEvent>> {
        Ok(vec![])
    }
}

impl AssetLoaderSystem {
    pub fn new() -> Self {
        Self {}
    }
}
