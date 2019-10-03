use std::slice::Iter;
use std::vec::Vec;

use super::{Color3, light, SceneObject};

pub struct Scene {
	objects: Vec<SceneObject>,
	lights: Vec<light::Lights>,
	skylight: Color3,
}

impl Scene {
	pub fn new(objs: Option<Vec<SceneObject>>, lights: Option<Vec<light::Lights>>, skylight: Option<Color3>)
		-> Scene {
		Scene {
			objects: objs.unwrap_or_else(Vec::new),
			lights: lights.unwrap_or_else(Vec::new),
			skylight: skylight.unwrap_or_else(|| Color3::new(0.0, 0.0, 0.0)),
		}
	}
	
	/*
	pub fn add_obj(&mut self, obj: SceneObject<'a>) {
		self.objects.push(obj);
	}
	
	pub fn add_light(&mut self, light: impl light::Light + 'a) {
		// TODO: move in instate of static
		self.lights.push(Box::new(light));
	}
	*/
	
	pub fn iter_obj(&self) -> Iter<SceneObject> {
		self.objects.iter()
	}
	
	pub fn iter_light(&self) -> Iter<light::Lights> {
		self.lights.iter()
	}
	
	pub fn get_skylight(&self) -> Color3 {
		self.skylight
	}
	
}