use egui::Color32;

use crate::{math::*, scene::Scene};

use self::raycast::*;

pub mod raycast;

pub fn raytracer(scene: &Scene, ray: &Ray) -> Color32 {
    let raycast_result = raycast_object_slice(&scene.objects, ray);

    if let Some(raycast_hit) = raycast_result {
        Color32::from_rgb(
            ((raycast_hit.distance - 1.3) * 400.0) as u8,
            0,
            ((raycast_hit.distance - 1.3) * 400.0) as u8,
        )
    } else {
        Color32::GRAY
    }
}
