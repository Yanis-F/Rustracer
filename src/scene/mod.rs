use crate::{color::RGB, math::*};

pub mod object;
use object::*;
pub mod light;
use light::*;

pub mod surface;

// Will NOT do:
// Light refraction
// Complex Mesh
// Texture

#[derive(Default, Clone)]
pub struct Scene {
    pub camera: Camera,
    pub ambiant: RGB,

    pub objects: Vec<SceneObject>,
    pub lights: Vec<SceneLight>,
}

#[derive(Default, Clone)]
pub struct Camera {
    pub position: Vector3,
    pub orientation: Quaternion,
}
