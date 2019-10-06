use std::f32::consts::FRAC_PI_4;
use std::time::Instant;

use nalgebra::{Point3, Rotation3, Unit, UnitQuaternion, Vector2, Vector3};

use rtracer::Color3;
use rtracer::geometric::{Disc, InfinitePlane, Sphere};
use rtracer::light::{AreaLight, DirectionalLight, PointLight};
use rtracer::renderer::{render, RenderImage};
use rtracer::SceneObject;

use crate::rtracer::SceneData;

mod rtracer;

fn main() {
	let scene_data = setup();
	rtracer::parser::save_scene_data("data.ron", &scene_data).unwrap();
	let scene=  rtracer::parser::load_scene_data("data.ron").unwrap();
	test_render(&scene);
}

fn setup() -> SceneData {

	use crate::rtracer::material;

	let camera = rtracer::Camera::new(
		Point3::new(0.0, 0.0, 0.0),
		Rotation3::from_euler_angles(0.0, 0.0, 0.0)
	);

	// let sphere = SceneObject::new(Sphere {pos: Point3::new(2.0, 0.5, 0.5), radius: 0.6});
	let sphere2 =
		SceneObject::new(
			Sphere {pos: Point3::new(3.0, 0.0, 0.0), radius: 1.0},
			material::Diffuse::new([1.0, 0.5, 0.5].into())
		);
	let floor =
		SceneObject::new(
			InfinitePlane {
				pos: Point3::new(1.0, 1.0, -1.0),
				norm: Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0))
			},
			material::Reflective::new(0.1)
		);/*
	let disc = SceneObject::new(Disc::new(
		Point3::new(1.0, 1.0, -1.0),
		Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0)),
		5.0));
	let light = PointLight::new(Point3::new(0.0, 1.0, 0.5), Color3::new(1.0, 1.0, 1.0));
	let light2 = PointLight::new(Point3::new(0.0, -1.0, 0.5), Color3::new(1.0, 1.0, 1.0));
	let sun = DirectionalLight::new(
		Unit::new_normalize(Vector3::new(0.0, -1.0, -1.0)),
		Color3::new(0.1, 0.1, 0.1)
	);*/
	let area = AreaLight::new(
		Vector3::new(0.0, 1.0, 1.0),
		UnitQuaternion::from_euler_angles(0.0, 0.0, -FRAC_PI_4),
		0.5,
		None
	);

	let scene = rtracer::Scene::from_maybe_component(
		Some(vec![floor, sphere2]),
		Some(vec![
			// uncomment to enable light
			// light.into(),
			// light2.into(),
			// sun.into(),
			area.into(),
		]),
		None
	);

	SceneData {scene, camera}
}

fn test_render(scene_data: &rtracer::SceneData) {
	// render
	const IMAGE_SIZE: u32 = 250;
	const VIEWPORT_SIZE: u32 = 2;
	const UNIT_PER_PIXEL: f32 = VIEWPORT_SIZE as f32 / IMAGE_SIZE as f32;

	let (scene, camera) = (&scene_data.scene, &scene_data.camera);

	print!("Start Rendering...");
	let start_time = Instant::now();
	let rendered_image: RenderImage = render(scene, camera, IMAGE_SIZE, UNIT_PER_PIXEL);
	let duration = start_time.elapsed();
	println!("\nRendering Finish In {:.2}s", duration.as_secs_f32());
	
	// save
	const FILE_NAME: &str = "render.png";
	rendered_image.save(FILE_NAME).expect("Unable to save Image");
	println!("Saving to {}", FILE_NAME);
}

