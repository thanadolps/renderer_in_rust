use nalgebra::{Point3, Unit, Vector3};

pub struct HitInfo {
	pub dist: f32,
	pub intersection: Point3<f32>,
	pub normal: Unit<Vector3<f32>>,
}