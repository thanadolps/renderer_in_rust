use super::shape::Shape;

pub struct SceneObject<'a> {
	// material: f32,
	pub shape: Box<dyn Shape + 'a>,
}

impl<'a> SceneObject<'a> {
	pub fn new<T: Shape + 'a> (shape: T) -> Self {
		SceneObject {
			shape: Box::new(shape),
		}
	}
}