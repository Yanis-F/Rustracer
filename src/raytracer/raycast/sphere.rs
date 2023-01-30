use crate::{math::*, scene::object::object_type::Sphere};

use super::RaycastHit;

pub fn raycast_sphere<'sphere>(sphere: &'sphere Sphere, ray: &Ray) -> Option<RaycastHit<'sphere>> {
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html

    let sphere_position_relative = vec3_sub(sphere.center, ray.origin);

    let tca = vec3_dot(sphere_position_relative, ray.direction);

    if tca < 0.0 {
        return None;
    }

    let l = vec3_len(sphere_position_relative);
    let d = (l * l - tca * tca).sqrt();

    if d > sphere.radius {
        return None;
    }

    let thc = (sphere.radius * sphere.radius - d * d).sqrt();

    let distance = tca - thc;
    let position = ray.get_position_at_distance(distance);
    let normal = vec3_normalized_sub(position, sphere.center);

    Some(RaycastHit {
        distance,
        surface: &sphere.surface,
        hit_direction: ray.direction,
        position,
        normal,
    })
}
