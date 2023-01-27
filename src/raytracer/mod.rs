use crate::{math::*, scene::Scene, color::RGB};

use self::raycast::*;

pub mod raycast;

pub fn raytracer(scene: &Scene, ray: &Ray) -> RGB {
    let raycast_result = raycast_object_slice(&scene.objects, ray);

    if let Some(raycast_hit) = raycast_result {
		raycast_hit.surface.ambiant.clone()
    } else {
        RGB::GRAY
    }
}
