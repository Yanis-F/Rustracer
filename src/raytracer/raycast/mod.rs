mod sphere;
pub use sphere::raycast_sphere;

use crate::{math::Ray, scene::object::SceneObject};

pub struct RaycastHit {
    pub distance: f32,
    // position, normal, distance, surface
}

pub fn raycast_object_slice(object_slice: &[SceneObject], ray: &Ray) -> Option<RaycastHit> {
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

pub fn raycast_object(object: &SceneObject, ray: &Ray) -> Option<RaycastHit> {
    match object {
        SceneObject::Sphere(sphere) => raycast_sphere(sphere, ray),
    }
}
