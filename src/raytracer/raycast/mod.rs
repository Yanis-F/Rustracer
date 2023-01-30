mod sphere;
pub use sphere::raycast_sphere;
mod plane;
pub use plane::raycast_plane;

use crate::{
    math::*,
    scene::{object::SceneObject, surface::Surface},
};

pub struct RaycastHit<'scene> {
    pub distance: f64,
    pub surface: &'scene Surface,
    pub position: Vector3,
    pub hit_direction: Vector3,
    pub normal: Vector3,
}

pub fn raycast_object_slice<'scene>(
    object_slice: &'scene [SceneObject],
    ray: &Ray,
) -> Option<RaycastHit<'scene>> {
    let mut result = None as Option<RaycastHit>;

    for object in object_slice {
        let raycast_result = raycast_object(object, ray);

        if let Some(raycast_hit) = raycast_result {
            if let Some(previous_hit) = &result {
                if raycast_hit.distance < previous_hit.distance {
                    result = Some(raycast_hit)
                }
            } else {
                result = Some(raycast_hit)
            }
        }
    }

    result
}

pub fn raycast_object<'scene>(
    object: &'scene SceneObject,
    ray: &Ray,
) -> Option<RaycastHit<'scene>> {
    match object {
        SceneObject::Sphere(sphere) => raycast_sphere(sphere, ray),
        SceneObject::Plane(plane) => raycast_plane(plane, ray),
    }
}
