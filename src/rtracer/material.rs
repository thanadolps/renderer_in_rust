use nalgebra::{Reflection, Unit, Vector3};
use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;
use rand_distr::Distribution;
use serde::{Deserialize, Serialize};

use enum_dispatch::enum_dispatch;

use crate::rtracer::{Color3, helper, HitInfo, light::Light, Scene, SceneObject};

#[enum_dispatch]
pub trait Material {
    fn compute_light(
        &self,
        scene: &Scene,
        hit_info: &HitInfo,
        hit_object: &SceneObject,
        rng: &mut impl Rng)
        -> Color3;
}

#[enum_dispatch(Material)]
#[derive(Serialize, Deserialize)]
pub enum Materials {
    Diffuse,
    Reflective,
}

#[derive(Serialize, Deserialize)]
pub struct Diffuse {
    color: Color3,
}

impl Diffuse {
    pub fn new(color: Color3) -> Self {
        Diffuse {color}
    }
}

impl Material for Diffuse {
    fn compute_light(&self, scene: &Scene, hit_info: &HitInfo,
                     hit_object: &SceneObject, rng: &mut impl Rng)
        -> Color3 {
        scene.iter_light()
            .map(|x| x.direct_light_at(hit_info.intersection, hit_info.normal, scene))
            .sum::<Color3>()
            .component_mul(&self.color)  // factor in material's color
    }
}

#[derive(Serialize, Deserialize)]
pub struct Reflective {
    fuzziness: f32,
}

impl Reflective {
    pub fn new(fuzziness: f32) -> Self {
        Reflective {fuzziness}
    }
}

impl Material for Reflective {
    fn compute_light(&self, scene: &Scene, hit_info: &HitInfo,
                     hit_object: &SceneObject, rng: &mut impl Rng) -> Color3 {

        use crate::rtracer::renderer::raycast_compute_light;
        use rand_distr::UnitBall;

        let reflection_noise = self.fuzziness * Vector3::from(UnitBall.sample(rng));
        let reflect_dir =
            Unit::new_normalize(
                helper::calculate_reflect_ray(
                    &hit_info.incoming_dir,
                    &hit_info.normal).into_inner()
                    + reflection_noise
            );
        raycast_compute_light(
            scene,
            hit_info.intersection.clone(),
            reflect_dir,
            rng
        )
    }
}