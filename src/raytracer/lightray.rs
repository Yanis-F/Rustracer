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
    }
}

fn get_directional_light_ray_to_surface(
    scene: &Scene,
    light: &light_type::Directional,
    hit: &RaycastHit,
) -> Option<LightRay> {
    let light_direction_normalized = vec3_normalized(light.direction);

    if vec3_dot(light_direction_normalized, hit.normal) > 0.0 {
        return None;
    }

    let inverse_ray = Ray {
        origin: hit.position,
        direction: vec3_neg(light_direction_normalized),
    };

    let raycast_result = raycast::raycast_object_slice(&scene.objects, &inverse_ray);

    if raycast_result.is_some() {
        return None; // no transparent object yet
    }

    Some(LightRay {
        direction: light_direction_normalized,
        color: light.color.clone(),
    })
}
