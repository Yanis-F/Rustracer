use crate::{math::*};

pub mod object;
use object::*;

pub mod surface;

// Will NOT do:
// Light refraction
// Complex Mesh
// Texture

#[derive(Default, Clone)]
pub struct Scene {
    pub camera: Camera,
    // ambiant light
    pub objects: Vec<SceneObject>,
    // Lights:
    // DirectionalLight
    // PointLight
}

#[derive(Default, Clone)]
pub struct Camera {
    pub position: Vector3,
    pub orientation: Quaternion,
}
