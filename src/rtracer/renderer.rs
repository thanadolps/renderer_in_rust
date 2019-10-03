use super::scene::Scene;
use super::Camera;
// use super::shape::Shape;
use super::Color3;
use super::HitInfo;
use nalgebra::{Vector3, Point3, Unit};
use image::ImageBuffer;
use image::Rgb;
use image::imageops::{dither, BiLevel, blur};
use image::math::nq::NeuQuant;
use itertools::Itertools;
use itertools::MinMaxResult::*;
use std::cmp::Ordering::Equal;


pub type RenderImage = ImageBuffer<Rgb<u8>, Vec<u8>>;
type RenderBuffer = ImageBuffer<Rgb<f32>, Vec<f32>>;


pub fn render(scene: &Scene, camera: &Camera, image_size: u32, unit_per_pixel: f32) -> RenderImage {
	
	let mut img: RenderBuffer = ImageBuffer::new(image_size, image_size);
	
	let half_width = img.width()/2;
	let half_height = img.height()/2;
	
	for (px, py, pixel) in img.enumerate_pixels_mut() {
		// get ray from camera
		let ray_dir = camera.ray_at_pixel_position(px, py, unit_per_pixel, half_width, half_height);
		
		// raycast!
		match raycast(scene, camera.pos, ray_dir) {
			None => *pixel = {
				let sky = scene.get_skylight();
				Rgb([sky[0], sky[1], sky[2]])
			},
			Some(hit) => {
				let combined_light: Color3 = scene.iter_light()
					 .map(|x| x.light_at(hit.intersection, hit.normal, scene) /* TODO: add brdf term */)
					 .sum();
				// TODO: change to directly store Color3
				*pixel = Rgb([combined_light[0], combined_light[1], combined_light[2]]);
			}
		}
	}
	
	// map from f32 image to u8 image
	color_map(img, Some(0.0), None)
	// color_map(img, None, None)
	// TODO: post process with dither and blur
}


// TODO: maybe change to input array of Color3?
fn color_map(buf: RenderBuffer, vmin: Option<f32>, vmax: Option<f32>) -> RenderImage {
	
	// calculate min, max of image
	let (min, max) = match (vmin, vmax) {
		(None, None) =>
			match buf.iter().minmax_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal)) {
				NoElements => panic!("Blank Image Provided!"),
				OneElement(_a) => panic!("MonoColor Image Provided Without vmin, vmax"),
				MinMax(&a, &b) => (a, b),
			},
		(None, Some(max)) => (*buf.iter().min_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal)).unwrap(), max),
		(Some(min), None) => (min, *buf.iter().max_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal)).unwrap()),
		(Some(min), Some(max)) => (min, max),
	};
	
	// copy and convert pixel from buf to img
	let mut img: RenderImage = ImageBuffer::new(buf.width(), buf.height());
	for (b, p) in buf.pixels().zip(img.pixels_mut()) {
		*p = Rgb([
			(255.0*(b[0] - min)/(max-min)) as u8,
			(255.0*(b[1] - min)/(max-min)) as u8,
			(255.0*(b[2] - min)/(max-min)) as u8
		]);
	}
	
	img
}

pub fn raycast(scene: &Scene, origin: Point3<f32>, dir: Unit<Vector3<f32>>) -> Option<HitInfo> {
	scene.iter_obj().map(|x| x.shape.intersect(origin, dir))
					.filter(|x| !x.is_none())
					.map(|x| x.unwrap())
					.filter(|x| x.dist > 1e-6)
					.min_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap_or(Equal))
}