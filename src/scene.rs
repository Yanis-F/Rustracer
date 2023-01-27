use crate::math::*;

#[derive(Default, Clone)]
pub struct Scene {
    pub camera: Camera,
}

#[derive(Default, Clone)]
pub struct Camera {
    pub position: Vector3,
    pub orientation: Quaternion,
}
