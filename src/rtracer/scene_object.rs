use super::shape::geometric::Shapes;

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