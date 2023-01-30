use crate::{math::*, scene::object::object_type::Plane};

use super::RaycastHit;

pub fn raycast_plane<'plane>(plane: &'plane Plane, ray: &Ray) -> Option<RaycastHit<'plane>> {
    let dot = vec3_dot(plane.normal, ray.direction);

    if dot >= 0.0 {
        return None;
    }

    let shortest_distance_from_ray_origin_to_plane =
        vec3_dot(vec3_sub(plane.point, ray.origin), plane.normal);

    let distance = shortest_distance_from_ray_origin_to_plane / dot;
    let position = ray.get_position_at_distance(distance);

    Some(RaycastHit {
        distance,
        surface: &plane.surface,
        position,
        normal: plane.normal,
    })
}
