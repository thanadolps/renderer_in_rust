use nalgebra::{Point3, Vector3, Unit};

pub struct HitInfo {
	pub dist: f32,
	pub intersection: Point3<f32>,
	pub normal: Unit<Vector3<f32>>,
}