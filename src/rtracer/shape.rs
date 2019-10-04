use nalgebra::{Point3, Unit, Vector3};

use enum_dispatch::enum_dispatch;

use super::HitInfo;

#[enum_dispatch]
pub trait Shape {
	fn intersect(&self, origin: Point3<f32>, dir: Unit<Vector3<f32>>) -> Option<HitInfo>;
}

pub mod geometric {
    use nalgebra::{Point3, Unit, Vector3};
    use serde::{Deserialize, Serialize};

    use enum_dispatch::enum_dispatch;

    use super::HitInfo;
    use super::Shape;

    #[enum_dispatch(Shape)]
	#[derive(Serialize, Deserialize)]
	pub enum Shapes {
		Sphere,
		InfinitePlane,
		Disc
	}

	#[derive(Serialize, Deserialize)]
	pub struct Sphere {
		pub pos: Point3<f32>,
		pub radius: f32
	}
	
	impl Shape for Sphere {
		fn intersect(&self, origin: Point3<f32>, dir: Unit<Vector3<f32>>) -> Option<HitInfo> {
			// http://viclw17.github.io/2018/07/16/raytracing-ray-sphere-intersection/
			// + optimization: a = 1 always (self dot = mag, mag of unit vector = 1) 
			let dac = origin - self.pos;
			let b = 2.0 * dir.dot(&dac);
			let c = dac.magnitude_squared() - self.radius * self.radius;
			
			let discriminant = b*b - 4.0*c;
			
			// if not intersecting
			if discriminant < 0.0 {
				return None;
			}
			
			let dist = (-b-discriminant.sqrt())/2.0;
			
			// if behide camera
			if dist <= 1e-6 {
				return None;
			}
			
			let intersection = origin + dir.into_inner() * dist;
			let normal = Unit::new_normalize(intersection - self.pos);
			
			Some(HitInfo {
				dist,
				intersection,
				normal,
			})
		}
	}

	#[derive(Serialize, Deserialize)]
	pub struct InfinitePlane {
		pub pos: Point3<f32>,
		pub norm: Unit<Vector3<f32>>,
		
	}
	
	impl InfinitePlane {
		fn _intersect(
			self_pos: Point3<f32>,
			self_norm: Unit<Vector3<f32>>, 
			origin: Point3<f32>,
			dir: Unit<Vector3<f32>>
		) -> Option<HitInfo> {
			let deno = dir.dot(self_norm.as_ref());
			
			if deno > -1e-4 {
				return None;
			}
			
			let dist = ((self_pos - origin).dot(self_norm.as_ref()))/deno;
			
			if dist > 0.0 { 
				Some(HitInfo {
					dist,
					intersection: origin + dir.as_ref() * dist,
					normal: self_norm
				})
			}
			else {
				None
			}
		}
	}
	
	impl Shape for InfinitePlane {
		fn intersect(&self, origin: Point3<f32>, dir: Unit<Vector3<f32>>) -> Option<HitInfo> {
			InfinitePlane::_intersect(self.pos, self.norm, origin, dir)
		}
	}

	#[derive(Serialize, Deserialize)]
	pub struct Disc {
		pub pos: Point3<f32>,
		pub norm: Unit<Vector3<f32>>,
		pub r_sq: f32,
	}
	
	impl Disc {
		pub fn new(pos: Point3<f32>, norm: Unit<Vector3<f32>>, r: f32) -> Self {
			Disc { pos, norm, r_sq: r*r }
		}
	}
	
	impl Shape for Disc {
		fn intersect(&self, origin: Point3<f32>, dir: Unit<Vector3<f32>>) -> Option<HitInfo> {
			let hit_info = InfinitePlane::_intersect(self.pos, self.norm, origin, dir);
			if let Some(hit) = hit_info {
				if (hit.intersection - self.pos).norm_squared() < self.r_sq {
					return Some(hit);
				}
				return None;
			}
			None
		}
	}
}