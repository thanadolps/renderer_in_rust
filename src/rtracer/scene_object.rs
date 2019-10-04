use serde::{Deserialize, Serialize};

use super::shape::geometric::Shapes;

#[derive(Serialize, Deserialize)]
pub struct SceneObject {
	// material: f32,
	pub shape: Shapes,
}

impl SceneObject {
	pub fn new (shape: impl Into<Shapes>) -> Self {
		SceneObject {
			shape: shape.into(),
		}
	}
}

impl From<Shapes> for SceneObject {
	fn from(shape: Shapes) -> Self {
		SceneObject::new(shape)
	}
}