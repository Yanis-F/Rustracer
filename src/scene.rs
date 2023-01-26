use crate::types::*;

#[derive(Default, Clone)]
pub struct Scene {
    pub camera: Camera,
}

#[derive(Default, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub orientation: Quaternion,
}
