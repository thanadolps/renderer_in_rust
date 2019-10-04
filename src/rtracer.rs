use nalgebra::base::Vector3;

pub use camera::Camera;
pub use hitinfo::HitInfo;
pub use parser::SceneData;
pub use parser::serde_interface;
pub use scene::Scene;
pub use scene_object::SceneObject;
pub use shape::geometric;
pub use shape::Shape;

mod scene;
pub mod light;
mod camera;
mod scene_object;
mod shape;
mod hitinfo;
pub mod renderer;
pub mod parser;

pub type Color3 = Vector3<f32>;

// number of sample use in monte carlo ray tracing of area light
// number of ray casted = AREALIGHT_MONTECARLO_SAMPLE
const AREALIGHT_MONTECARLO_SAMPLE: u32 = 121;

// square root of number of grid (sample) in finite differential ray tracing of area light
// number of ray casted = (2*AREALIGHT_FINITEDIFF_LENGTH + 1)^2
const AREALIGHT_FINITEDIFF_LENGTH: u32 = 4;

/*
Coordinate System
	Base Axis (when no rotation apply)
		+x = forward
		+y = right
		+z = up
*/