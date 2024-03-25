use super::engine::TOTAL_ENTITIES;

pub trait Component {}

pub struct ComponentsData {
    render_components: Vec<Option<RenderComponent>>,
}

pub struct RenderComponent {}
pub struct TransformComponent {}


impl ComponentsData {
	pub fn new() -> Self {
		Self {
			render_components: Vec::with_capacity(TOTAL_ENTITIES)
		}
	}
}