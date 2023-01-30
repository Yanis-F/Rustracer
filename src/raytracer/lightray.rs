use crate::{
    color::Rgb,
    math::*,
    scene::{
        light::{light_type, SceneLight},
        Scene,
    },
};

use super::raycast::{self, RaycastHit};

pub struct LightRay {
    pub direction: Vector3,
    pub color: Rgb,
}

pub fn get_all_light_rays_hitting_surface(scene: &Scene, hit: &RaycastHit) -> Vec<LightRay> {
    scene
        .lights
        .iter()
        .flat_map(|light| get_light_ray_to_surface(scene, light, hit))
        .collect()
}

fn get_light_ray_to_surface(
    scene: &Scene,
    light: &SceneLight,
    hit: &RaycastHit,
) -> Option<LightRay> {
    match light {
        SceneLight::Directional(light) => get_directional_light_ray_to_surface(scene, light, hit),
        SceneLight::Point(light) => get_point_light_ray_to_surface(scene, light, hit),
    }
}

fn get_directional_light_ray_to_surface(
    scene: &Scene,
    light: &light_type::Directional,
    hit: &RaycastHit,
) -> Option<LightRay> {
	let direction = vec3_normalized(light.direction);
    get_perceived_light_color_at_hitpoint(scene, light.color, direction, hit).map(|color| {
        LightRay {
            direction,
            color,
        }
    })
}

fn get_point_light_ray_to_surface(
    scene: &Scene,
    light: &light_type::Point,
    hit: &RaycastHit,
) -> Option<LightRay> {
    let direction = vec3_normalized_sub(hit.position, light.position);

    get_perceived_light_color_at_hitpoint(scene, light.color, direction, hit)
        .map(|color| LightRay { direction, color })
}

/// Computes what is the final color of light perceived from this light source at the given point
/// This may include altering the color of the light when passing through transparent object
///
/// May be None if no light is perceived at this position
fn get_perceived_light_color_at_hitpoint(
    scene: &Scene,
    light_color: Rgb,
    lightray_direction: Vector3,
    hit: &RaycastHit,
) -> Option<Rgb> {
    if vec3_dot(lightray_direction, hit.normal) > 0.0 {
        return None;
    }

    let inverse_ray = Ray {
        origin: hit.position,
        direction: vec3_neg(lightray_direction),
    };

    let raycast_result = raycast::raycast_object_slice(&scene.objects, &inverse_ray);

    if raycast_result.is_some() {
        return None; // no transparent object yet
    }

    Some(light_color)
}
