use crate::{math::*, scene::Scene, color::RGB};

use self::raycast::*;

pub mod raycast;

const VOID_COLOR: RGB = RGB::GRAY;

pub fn raytracer(scene: &Scene, ray: &Ray) -> RGB {
    let raycast_result = raycast_object_slice(&scene.objects, ray);

	let raycast_hit = match raycast_result {
		Some(hit) => hit,
		None => return VOID_COLOR,
	};

	let light_color = &scene.ambiant;

	// find all RBGA light affecting hit spot
	// additon-synthesis of all light


	let light_color_rgb = light_color.to_rgb(VOID_COLOR);
	light_color_rgb

	// RGB::subtractive_synthesis(&raycast_hit.surface.ambiant, &light_color_rgb)
}
