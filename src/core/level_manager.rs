use super::engine::LevelManager;


pub struct StarterLevel {
}

impl LevelManager for StarterLevel {
    fn load_resources(&mut self) {
        
    }

    fn create_entities(&mut self, _entity_manager: &super::engine::EntityManagerRef) {
        
    }
}