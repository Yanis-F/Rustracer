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

	// TODO:
	// Light calculation:
	// First get a vector of <LightRay> of all lights reaching the raycast_hit point.
	// Each `LightRay` shall have a RBGA color, a direction
	// The ambiant light is handled separately
	//
	// The perceived Ambiant color is the subtractive_synthesis of ambiant light and
	// surface.ambiant.
	//
	// The perceived Diffused color is the subtractive_synthesis of all the `LightRay.color`
	// aggregated by additive synthesis.
	//
	// The perceived specular highlight is calculated with the angle at which the `LightRay` hits
	// the surface, as well as the specular color of the surface and the shininess of the surface
	//
	//
	// Final perceived color:
	// Aggregation by additional synthesis of perceived Ambiant color, perceived Diffused color and
	// perceived Specular highlight

	let perceived_ambiant_color = get_perceived_ambiant_color(&scene.ambiant, raycast_hit.surface);

	perceived_ambiant_color
}

fn get_perceived_ambiant_color(ambiant: &crate::color::RGBA, surface: &crate::scene::surface::Surface) -> RGB {
	let ambian_rgb = ambiant.to_rgb(VOID_COLOR);

	RGB::subtractive_synthesis(&surface.ambiant, &ambian_rgb)
}
