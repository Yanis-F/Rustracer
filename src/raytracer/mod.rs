use crate::{
    color::RGB,
    math::*,
    scene::{surface::Surface, Scene},
};

use self::{
    lightray::{get_all_light_rays_hitting_surface, LightRay},
    raycast::*,
};

pub mod lightray;
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

    let lightrays = get_all_light_rays_hitting_surface(scene, &raycast_hit);

    let perceived_ambiant_color = get_perceived_ambiant_color(&scene.ambiant, raycast_hit.surface);
    let perceived_diffuse_color = get_perceived_diffuse_color(&lightrays, &raycast_hit);

    RGB::additive_synthesis(&perceived_ambiant_color, &perceived_diffuse_color)
}

fn get_perceived_ambiant_color(ambiant: &RGB, surface: &Surface) -> RGB {
    RGB::subtractive_synthesis(&surface.ambiant, ambiant)
}

fn get_perceived_diffuse_color(lightrays: &[LightRay], hit: &RaycastHit) -> RGB {
    lightrays.iter().fold(RGB::BLACK, |accumulator, lightray| {
        let effectiveness = -vec3_dot(lightray.direction, hit.normal);
        let effective_light_color = lightray.color.scale(effectiveness as f32);
        let ray_diffuse_color =
            RGB::subtractive_synthesis(&effective_light_color, &hit.surface.diffuse);

        RGB::additive_synthesis(&accumulator, &ray_diffuse_color)
    })
}
