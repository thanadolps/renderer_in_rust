use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Result;

// TODO: Finish This

#[derive(Deserialize)]
struct V3V3 {
	pos: [f32; 3],
	rot: [f32; 3],
}

#[derive(Deserialize)]
struct V3S {
	pos: [f32; 3],
	val: f32,
}

type PosRot = V3V3;
type PosRad = V3S;
type PosNorm = V3V3;

#[derive(Deserialize)]
struct SceneObjectJSON {
	spheres: Vec<PosRad>,
	planes: Vec<PosNorm>,
}

#[derive(Deserialize)]
struct SceneJSONResult {
	camera : PosRot,
	objects: Vec<V3S>,
}


pub fn parse_json() {

	unimplemented!();

    let data = r#"
    {
        "camera": {
			"pos": [1.0, 2.0, 3.0],
			"rot": [-1.0, -2.0, -3.0]
		}
    }"#;
	
	let p: SceneJSONResult = serde_json::from_str(data).unwrap();
	
	for (i, j) in p.camera.pos.iter().zip(p.camera.rot.iter()) {
		println!("{} {}", i, j);
	}
	
}